use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::touch;
use iced::{Border, Element, Event, Length, Point, Rectangle, Shadow, Size};

/// Event produced by the ResizableTable when a column resize handle is dragged.
#[derive(Debug, Clone, Copy)]
pub enum TableMessage {
    Resized { column_index: usize, new_width: f32 },
}

/// A custom table widget with draggable column handles.
pub struct ResizableTable<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    headers: Vec<Element<'a, Message, Theme, Renderer>>,
    rows: Vec<Vec<Element<'a, Message, Theme, Renderer>>>,
    widths: Vec<f32>,
    min_width: f32,
    handle_width: f32,
    on_resize: Box<dyn Fn(usize, f32) -> Message + 'a>,
}

impl<'a, Message, Theme, Renderer> ResizableTable<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn new<F>(widths: impl Into<Vec<f32>>, on_resize: F) -> Self
    where
        F: Fn(usize, f32) -> Message + 'a,
    {
        Self {
            headers: Vec::new(),
            rows: Vec::new(),
            widths: widths.into(),
            min_width: 50.0,
            handle_width: 6.0,
            on_resize: Box::new(on_resize),
        }
    }

    pub fn headers(
        mut self,
        headers: Vec<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        self.headers = headers;
        self
    }

    pub fn rows(
        mut self,
        rows: Vec<Vec<Element<'a, Message, Theme, Renderer>>>,
    ) -> Self {
        self.rows = rows;
        self
    }

    pub fn min_width(mut self, min: f32) -> Self {
        self.min_width = min;
        self
    }
}

/// Internal state required to track dragging interactions across frames.
#[derive(Debug, Default)]
pub struct TableState {
    dragging_column: Option<(usize, f32)>, // (index, starting_x_position)
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ResizableTable<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        let mut total_width = 0.0;
        let mut max_header_height: f32 = 0.0;

        // Measure Header Nodes
        let mut header_nodes = Vec::new();
        for (i, header) in self.headers.iter_mut().enumerate() {
            let col_width = self.widths.get(i).copied().unwrap_or(100.0);
            total_width += col_width;

            let child_limits = layout::Limits::new(
                Size::new(col_width, 0.0),
                Size::new(col_width, f32::INFINITY),
            );

            let node = header.as_widget_mut().layout(&mut tree.children[i], renderer, &child_limits);
            max_header_height = max_header_height.max(node.size().height);
            header_nodes.push(node);
        }

        // Layout rows
        let mut row_nodes = Vec::new();
        let mut current_y = max_header_height;

        let row_children_offset = self.headers.len();
        let mut tree_idx = row_children_offset;

        for row in &mut self.rows {
            let mut row_max_height: f32 = 0.0;
            let mut col_nodes = Vec::new();
            let mut x_offset = 0.0;

            for (col_idx, cell) in row.iter_mut().enumerate() {
                let col_width = self.widths.get(col_idx).copied().unwrap_or(100.0);
                let child_limits = layout::Limits::new(
                    Size::new(col_width, 0.0),
                    Size::new(col_width, f32::INFINITY),
                );

                let mut node = cell.as_widget_mut().layout(&mut tree.children[tree_idx], renderer, &child_limits);
                node = node.move_to(Point::new(x_offset, current_y));
                row_max_height = row_max_height.max(node.size().height);

                col_nodes.push(node);
                x_offset += col_width;
                tree_idx += 1;
            }

            current_y += row_max_height;
            row_nodes.push(layout::Node::with_children(
                Size::new(total_width, row_max_height),
                col_nodes,
            ));
        }

        // Position headers at top
        let mut x_offset = 0.0;
        let positioned_headers: Vec<layout::Node> = header_nodes
            .into_iter()
            .zip(&self.headers)
            .enumerate()
            .map(|(i, (node, _))| {
                let col_width = self.widths.get(i).copied().unwrap_or(100.0);
                let pos_node = node.move_to(Point::new(x_offset, 0.0));
                x_offset += col_width;
                pos_node
            })
            .collect();

        let mut all_children = positioned_headers;
        all_children.extend(row_nodes);

        layout::Node::with_children(Size::new(total_width, current_y), all_children)
    }

    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<TableState>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(TableState::default())
    }

    fn children(&self) -> Vec<widget::Tree> {
        let mut children = Vec::new();
        for header in &self.headers {
            children.push(widget::Tree::new(header));
        }
        for row in &self.rows {
            for cell in row {
                children.push(widget::Tree::new(cell));
            }
        }
        children
    }

    fn diff(&self, tree: &mut widget::Tree) {
        let mut children = Vec::new();
        for header in &self.headers {
            children.push(header);
        }
        for row in &self.rows {
            for cell in row {
                children.push(cell);
            }
        }
        tree.diff_children(&children);
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<TableState>();
        let bounds = layout.bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(cursor_pos) = cursor.position_in(bounds) {
                    let mut current_x = 0.0;
                    for (i, &w) in self.widths.iter().enumerate() {
                        current_x += w;
                        let handle_bounds = Rectangle {
                            x: current_x - (self.handle_width / 2.0),
                            y: 0.0,
                            width: self.handle_width,
                            height: bounds.height,
                        };

                        if handle_bounds.contains(cursor_pos) {
                            state.dragging_column = Some((i, cursor_pos.x));
                            shell.capture_event();
                            return;
                        }
                    }
                }
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                if state.dragging_column.is_some() {
                    state.dragging_column = None;
                    shell.capture_event();
                    return;
                }
            }

            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if let Some((col_idx, _)) = state.dragging_column {
                    let bounds_pos = bounds.position();
                    let relative_x = position.x - bounds_pos.x;

                    let left_offset: f32 = self.widths.iter().take(col_idx).sum();
                    let new_width = (relative_x - left_offset).max(self.min_width);

                    shell.publish((self.on_resize)(col_idx, new_width));
                    shell.capture_event();
                    return;
                }
            }
            _ => {}
        }

        // Delegate remaining events to child elements
        let mut children = layout.children();

        // Process headers
        for (i, header) in self.headers.iter_mut().enumerate() {
            if let Some(child_layout) = children.next() {
                header.as_widget_mut().update(
                    &mut tree.children[i],
                    event,
                    child_layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                );
            }
        }

        // Process row cells
        let mut tree_idx = self.headers.len();
        for row in self.rows.iter_mut() {
            if let Some(row_layout) = children.next() {
                let mut cell_layouts = row_layout.children();
                for cell in row.iter_mut() {
                    if let Some(cell_layout) = cell_layouts.next() {
                        cell.as_widget_mut().update(
                            &mut tree.children[tree_idx],
                            event,
                            cell_layout,
                            cursor,
                            renderer,
                            clipboard,
                            shell,
                            viewport,
                        );
                        tree_idx += 1;
                    }
                }
            }
        }
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<TableState>();
        let mut children = layout.children();

        // Colors adapt automatically to light & dark themes using text_color alpha
        let grid_line_color = style.text_color.scale_alpha(0.15);
        let header_line_color = style.text_color.scale_alpha(0.35);
        let handle_active_color = style.text_color.scale_alpha(0.60);

        // Draw headers
        for (i, header) in self.headers.iter().enumerate() {
            if let Some(child_layout) = children.next() {
                header.as_widget().draw(
                    &tree.children[i],
                    renderer,
                    theme,
                    style,
                    child_layout,
                    cursor,
                    viewport,
                );
            }
        }

        // Draw rows
        let mut tree_idx = self.headers.len();
        for row in &self.rows {
            if let Some(row_layout) = children.next() {
                let mut cell_layouts = row_layout.children();
                for cell in row {
                    if let Some(cell_layout) = cell_layouts.next() {
                        cell.as_widget().draw(
                            &tree.children[tree_idx],
                            renderer,
                            theme,
                            style,
                            cell_layout,
                            cursor,
                            viewport,
                        );
                        tree_idx += 1;
                    }
                }
            }
        }

        // --- GRID LINES & RESIZE HANDLES ---

        // Re-read child layout bounds for horizontal dividers
        let mut layout_children = layout.children();

        // 3. Draw Header Bottom Border (Horizontal)
        let mut header_height: f32 = 0.0;
        for _ in 0..self.headers.len() {
            if let Some(child_layout) = layout_children.next() {
                header_height = header_height.max(child_layout.bounds().height);
            }
        }

        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.y + header_height,
                    width: bounds.width,
                    height: 1.0,
                },
                border: Border::default(),
                shadow: Shadow::default(),
                snap: true,
            },
            header_line_color,
        );

        // 4. Draw Row Dividers (Horizontal)
        let mut current_y = header_height;
        for _ in &self.rows {
            if let Some(row_layout) = layout_children.next() {
                current_y += row_layout.bounds().height;

                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: bounds.x,
                            y: bounds.y + current_y,
                            width: bounds.width,
                            height: 1.0,
                        },
                        border: Border::default(),
                        shadow: Shadow::default(),
                        snap: true,
                    },
                    grid_line_color,
                );
            }
        }

        // 5. Draw Column Dividers & Resizer Handles (Vertical)
        let cursor_pos = cursor.position_in(bounds);
        let mut current_x = 0.0;

        for (i, &col_width) in self.widths.iter().enumerate() {
            current_x += col_width;

            if i < self.widths.len() - 1 {
                let handle_bounds = Rectangle {
                    x: bounds.x + current_x - (self.handle_width / 2.0),
                    y: bounds.y,
                    width: self.handle_width,
                    height: bounds.height,
                };

                let is_dragging = state.dragging_column.map_or(false, |(col, _)| col == i);
                let is_hovered = cursor_pos.map_or(false, |p| handle_bounds.contains(p));

                // If hovered or actively dragged, draw a prominent handle line
                if is_dragging || is_hovered {
                    renderer.fill_quad(
                        Quad {
                            bounds: Rectangle {
                                x: bounds.x + current_x - 1.0,
                                y: bounds.y,
                                width: 2.0,
                                height: bounds.height,
                            },
                            border: Border::default(),
                            shadow: Shadow::default(),
                            snap: true,
                        },
                        handle_active_color,
                    );
                } else {
                    // Regular subtle column grid line
                    renderer.fill_quad(
                        Quad {
                            bounds: Rectangle {
                                x: bounds.x + current_x - 0.5,
                                y: bounds.y,
                                width: 1.0,
                                height: bounds.height,
                            },
                            border: Border::default(),
                            shadow: Shadow::default(),
                            snap: true,
                        },
                        grid_line_color,
                    );
                }
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<TableState>();

        if state.dragging_column.is_some() {
            return mouse::Interaction::ResizingHorizontally;
        }

        if let Some(cursor_pos) = cursor.position_in(layout.bounds()) {
            let mut current_x = 0.0;
            for &w in self.widths.iter() {
                current_x += w;
                let handle_bounds = Rectangle {
                    x: current_x - (self.handle_width / 2.0),
                    y: 0.0,
                    width: self.handle_width,
                    height: layout.bounds().height,
                };

                if handle_bounds.contains(cursor_pos) {
                    return mouse::Interaction::ResizingHorizontally;
                }
            }
        }

        mouse::Interaction::default()
    }
}

impl<'a, Message, Theme, Renderer> From<ResizableTable<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(table: ResizableTable<'a, Message, Theme, Renderer>) -> Self {
        Element::new(table)
    }
}
