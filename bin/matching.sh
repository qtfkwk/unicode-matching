#!/usr/bin/env bash

# Prints the Rust source for a constant containing the open/close graphemes for brackets/quotes.
#
# - Punctuation, Open (Ps)
# - Punctuation, Close (Pe)
# - Punctuation, Initial Quote (Pi)
# - Punctuation, Final Quote (Pf)
#
# Ref: https://stackoverflow.com/a/13535289/21883239

cd $(dirname $0)

# Download the data file (if not present)
if [ ! -e UnicodeData.txt ]; then
    wget -i http://www.unicode.org/Public/UNIDATA/UnicodeData.txt
fi

I=0

printf "pub const MATCHING: &[(&str, &str)] = &[\n"

while IFS=';' read number name category rest
do
    if [[ "$category" =~ Ps|Pe|Pi|Pf ]]
    then
        # printf "%s (U+%s, %s): \u"$number"\n" "$name" "$number" "$category"

        if [ $((I % 2)) -eq 0 ] && [[ "$name" =~ LEFT ]]; then
            printf "    (\"\u"$number"\", "
            I=$((I+1))
        elif [ $((I % 2)) -eq 1 ] && [[ "$name" =~ RIGHT ]]; then
            printf "\"\u"$number"\"),\n"
            I=$((I+1))
        fi
    fi
done <UnicodeData.txt

printf "    // Extra:\n"
printf "    (\"<\", \">\"),\n"
printf "];\n"

