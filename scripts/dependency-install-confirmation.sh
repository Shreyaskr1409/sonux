#!/bin/bash



echo -e "\n"
# gstreamer
echo -e "Checking if gstreamer-1.0 is installed or not..."

echo -e "EXEC: pkg-config --cflags gstreamer-1.0"
file_list=$(pkg-config --cflags gstreamer-1.0)
echo "$file_list" | fold -w 70 | awk '{print "      "$0}'

echo -e "EXEC: awk -F ' ' '{print \$1}' ===FILELIST AS ARG==="
file_name=$(echo "$file_list" | awk -F ' ' '{print $1}')
echo "$file_name" | sed 's/^/      /'

file_name_trimmed=$(echo "$file_name" | sed 's/-I//')
file_name_edited="$file_name_trimmed/gst/gst.h"
ls $file_name_edited | sed 's/^/FILENAME: /'
file=$(ls $file_name_edited)

if [ -z "$file" ]; then
    echo "STATUS: Unavailable"
    echo "      libgstreamer1.0-dev not installed on this system. Install the dependency to continue using this application"
else
    echo "STATUS: Available"
    echo "      dependencies already installed on the system"
fi



echo -e "\n"
# taglib
echo -e "Checking if libtagc0 is installed or not..."

echo -e "EXEC: pkg-config --cflags taglib_c"
file_list=$(pkg-config --cflags taglib_c)
echo "$file_list" | fold -w 70 | awk '{print "      "$0}'

echo -e "EXEC: awk -F ' ' '{print \$1}' ===FILELIST AS ARG==="
file_name=$(echo "$file_list" | awk -F ' ' '{print $1}')
echo "$file_name" | sed 's/^/      /'

file_name_trimmed=$(echo "$file_name" | sed 's/-I//')
file_name_edited="$file_name_trimmed/tag_c.h"
ls $file_name_edited | sed 's/^/FILENAME: /'
file=$(ls $file_name_edited)

if [ -z "$file" ]; then
    echo "STATUS: Unavailable"
    echo "      libtagc0 not installed on this system. Install the dependency to continue using this application"
else
    echo "STATUS: Available"
    echo "      dependencies already installed on the system"
fi

echo -e "\n"
