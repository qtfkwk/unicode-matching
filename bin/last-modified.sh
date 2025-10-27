#!/usr/bin/bash

curl -sI http://www.unicode.org/Public/UNIDATA/UnicodeData.txt |sed -n 's/last-modified: //Ip' |sed 's/\r//g'

