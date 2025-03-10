#!/usr/bin/env perl

# Generate Rust source for constants containing the open/close brackets and functions that use them
#
# - Ps: Punctuation, Open
# - Pe: Punctuation, Close
# - Pi: Punctuation, Initial Quote
# - Pf: Punctuation, Final Quote
#
# Ref: https://stackoverflow.com/a/13535289/21883239
# Ref: https://stackoverflow.com/questions/13535172/list-of-all-unicodes-open-close-brackets/13535289#comment53701946_13535289

binmode(STDOUT, ":utf8");

use Cwd;
use File::Basename;
use File::Fetch;
use Text::Wrap;

# Wrap comment text to N columns
$Text::Wrap::columns = 100;

# Change to the directory with this file
my $dir = dirname($0);
chdir($dir) or die "Cannot change directory to $dir: $!";

# Download the data files (if not present)
sub download {
    my $url = shift;
    my $ff = File::Fetch->new(uri => $url);
    $ff->fetch() or die $ff->error;
}
for ("BidiBrackets", "BidiMirroring", "UnicodeData") {
    if (! -e "$_.txt") {
        download "https://www.unicode.org/Public/UNIDATA/$_.txt";
    }
}

# Parse UnicodeData.txt
my %unicodedata = ();
my $filename = "UnicodeData.txt";
open(my $fh, '<', $filename) or die "Cannot open $filename: $!";
my $i = 0;
while (my $line = <$fh>) {
    chomp $line;
    ($number, $name, $category, $rest) = split /;/, $line;
    if ($category =~ /^(Ps|Pe|Pi|Pf)$/) {
        $even = $i % 2 == 0;
        $odd = ! $even;
        if ($even && $name =~ /LEFT/) {
            $open = chr hex $number;
            $i++;
        } elsif ($odd && $name =~ /RIGHT/ && $open ne "") {
            $close = chr hex $number;
            $unicodedata{$open} = $close;
            $open = "";
            $close = "";
            $i++;
        } else {
            $open = "";
            $close = "";
        }
    }
}
close($fh);

# Parse BidiBrackets.txt
my %bidibrackets = ();
my $filename = "BidiBrackets.txt";
open(my $fh, '<', $filename) or die "Cannot open $filename: $!";
while (my $line = <$fh>) {
    chomp $line;
    $line =~ s/ #.*//;
    ($open, $close, $t) = split /; /, $line;
    if ($t eq "o") {
        $open = chr hex $open;
        $close = chr hex $close;
        $bidibrackets{$open} = $close;
    }
}
close($fh);

# Parse BidiMirroring.txt
my %bidimirroring = ();
my $filename = "BidiMirroring.txt";
open(my $fh, '<', $filename) or die "Cannot open $filename: $!";
while (my $line = <$fh>) {
    chomp $line;
    if ($line =~ /^([0-9A-F]{4}); ([0-9A-F]{4})/) {
        $open = chr hex $1;
        $close = chr hex $2;
        if (!exists $bidimirroring{$close}) {
            $bidimirroring{$open} = $close;
        }
    }
}
close($fh);

# Create combined sets
%brackets_matching = (%bidibrackets, %unicodedata);
%brackets_mirroring = (%bidibrackets, %bidimirroring);
%mirroring_matching = (%bidimirroring, %unicodedata);
%all = (%bidibrackets, %bidimirroring, %unicodedata);

# Print the Rust source for the functions
sub wrap_comment {
    my $comment = shift;
    if (length($comment) > $Text::Wrap::columns - 4) {
        $comment = wrap("", "", $comment);
        $comment = "/**\n$comment\n*/";
    } else {
        $comment = "/// $comment";
    }
    return $comment;
}
for (
    ["", "MATCHING", "`UnicodeData.txt`", \%unicodedata],
    ["_brackets", "BRACKETS", "`BidiBrackets.txt`", \%bidibrackets],
    ["_mirroring", "MIRRORING", "`BidiMirroring.txt`", \%bidimirroring],
    ["_brackets_matching", "BRACKETS_MATCHING", "`UnicodeData.txt` and `BidiBrackets.txt`", \%brackets_matching],
    ["_brackets_mirroring", "BRACKETS_MIRRORING", "`BidiMirroring.txt` and `BidiBrackets.txt`", \%brackets_mirroring],
    ["_mirroring_matching", "MIRRORING_MATCHING", "`UnicodeData.txt` and `BidiMirroring.txt`", \%mirroring_matching],
    ["_all", "ALL", "`UnicodeData.txt`, `BidiBrackets.txt`, and `BidiMirroring.txt`", \%all],
) {
    my ($name, $const, $files, $data) = @$_;

    # Format comments
    my $const_doc = wrap_comment("Matching open/close brackets from $files");
    my $close_doc = wrap_comment("Generate a [`BTreeMap`] with the matching close bracket for each open bracket in $files");
    my $open_doc = wrap_comment("Generate a [`BTreeMap`] with the matching open bracket for each close bracket in $files");
    my $matching_doc = wrap_comment("Generate a [`BTreeMap`] with an entry for each pair of opening and closing brackets in $files");

    # Print the constant
    print "$const_doc\npub const $const: &[(&str, &str)] = &[\n";
    foreach my $key (sort keys %$data) {
        print "    (\"$key\", \"$data->{$key}\"),\n";
    }
    print "];\n\n";

    # Print the close function
    print "$close_doc
pub fn close$name() -> BTreeMap<&'static str, &'static str> {
    $const.iter().cloned().collect()
}\n\n";

    # Print the open function
    print "$open_doc
pub fn open$name() -> BTreeMap<&'static str, &'static str> {
    $const
        .iter()
        .cloned()
        .map(|(open, close)| (close, open))
        .collect()
}\n\n";

    # Print the matching function
    print "$matching_doc
pub fn matching$name() -> BTreeMap<&'static str, &'static str> {
    $const
        .iter()
        .cloned()
        .flat_map(|(open, close)| [(open, close), (close, open)])
        .collect()
}\n\n";
}

