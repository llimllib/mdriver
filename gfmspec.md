# GitHub Flavored Markdown Spec

Version 0.29-gfm (2019-04-06)

This formal specification is based on the [CommonMark
Spec](http://spec.commonmark.org) by [John
MacFarlane](http://github.com/jgm) and licensed under
<a href="http://creativecommons.org/licenses/by-sa/4.0/"
rel="license"><img
src="https://i.creativecommons.org/l/by-sa/4.0/80x15.png"
style="border-width:0" alt="Creative Commons BY-SA" /></a>

- [<span class="number">1</span>Introduction](#introduction)
  - [<span class="number">1.1</span>What is GitHub Flavored
    Markdown?](#what-is-github-flavored-markdown-)
  - [<span class="number">1.2</span>What is
    Markdown?](#what-is-markdown-)
  - [<span class="number">1.3</span>Why is a spec
    needed?](#why-is-a-spec-needed-)
  - [<span class="number">1.4</span>About this
    document](#about-this-document)
- [<span class="number">2</span>Preliminaries](#preliminaries)
  - [<span class="number">2.1</span>Characters and
    lines](#characters-and-lines)
  - [<span class="number">2.2</span>Tabs](#tabs)
  - [<span class="number">2.3</span>Insecure
    characters](#insecure-characters)
- [<span class="number">3</span>Blocks and inlines](#blocks-and-inlines)
  - [<span class="number">3.1</span>Precedence](#precedence)
  - [<span class="number">3.2</span>Container blocks and leaf
    blocks](#container-blocks-and-leaf-blocks)
- [<span class="number">4</span>Leaf blocks](#leaf-blocks)
  - [<span class="number">4.1</span>Thematic breaks](#thematic-breaks)
  - [<span class="number">4.2</span>ATX headings](#atx-headings)
  - [<span class="number">4.3</span>Setext headings](#setext-headings)
  - [<span class="number">4.4</span>Indented code
    blocks](#indented-code-blocks)
  - [<span class="number">4.5</span>Fenced code
    blocks](#fenced-code-blocks)
  - [<span class="number">4.6</span>HTML blocks](#html-blocks)
  - [<span class="number">4.7</span>Link reference
    definitions](#link-reference-definitions)
  - [<span class="number">4.8</span>Paragraphs](#paragraphs)
  - [<span class="number">4.9</span>Blank lines](#blank-lines)
  - <span class="extension">[<span class="number">4.10</span>Tables
    (extension)](#tables-extension-)</span>
- [<span class="number">5</span>Container blocks](#container-blocks)
  - [<span class="number">5.1</span>Block quotes](#block-quotes)
  - [<span class="number">5.2</span>List items](#list-items)
  - <span class="extension">[<span class="number">5.3</span>Task list
    items (extension)](#task-list-items-extension-)</span>
  - [<span class="number">5.4</span>Lists](#lists)
- [<span class="number">6</span>Inlines](#inlines)
  - [<span class="number">6.1</span>Backslash
    escapes](#backslash-escapes)
  - [<span class="number">6.2</span>Entity and numeric character
    references](#entity-and-numeric-character-references)
  - [<span class="number">6.3</span>Code spans](#code-spans)
  - [<span class="number">6.4</span>Emphasis and strong
    emphasis](#emphasis-and-strong-emphasis)
  - <span class="extension">[<span class="number">6.5</span>Strikethrough
    (extension)](#strikethrough-extension-)</span>
  - [<span class="number">6.6</span>Links](#links)
  - [<span class="number">6.7</span>Images](#images)
  - [<span class="number">6.8</span>Autolinks](#autolinks)
  - <span class="extension">[<span class="number">6.9</span>Autolinks
    (extension)](#autolinks-extension-)</span>
  - [<span class="number">6.10</span>Raw HTML](#raw-html)
  - <span class="extension">[<span class="number">6.11</span>Disallowed
    Raw HTML (extension)](#disallowed-raw-html-extension-)</span>
  - [<span class="number">6.12</span>Hard line
    breaks](#hard-line-breaks)
  - [<span class="number">6.13</span>Soft line
    breaks](#soft-line-breaks)
  - [<span class="number">6.14</span>Textual content](#textual-content)
- [Appendix: A parsing strategy](#appendix-a-parsing-strategy)
  - [Overview](#overview)
  - [Phase 1: block structure](#phase-1-block-structure)
  - [Phase 2: inline structure](#phase-2-inline-structure)

# <span class="number">1</span>Introduction

## <a href="#TOC" class="toc-link"></a><span class="number">1.1</span>What is GitHub Flavored Markdown?

GitHub Flavored Markdown, often shortened as GFM, is the dialect of
Markdown that is currently supported for user content on GitHub.com and
GitHub Enterprise.

This formal specification, based on the CommonMark Spec, defines the
syntax and semantics of this dialect.

GFM is a strict superset of CommonMark. All the features which are
supported in GitHub user content and that are not specified on the
original CommonMark Spec are hence known as **extensions**, and
highlighted as such.

While GFM supports a wide range of inputs, it’s worth noting that
GitHub.com and GitHub Enterprise perform additional post-processing and
sanitization after GFM is converted to HTML to ensure security and
consistency of the website.

## <a href="#TOC" class="toc-link"></a><span class="number">1.2</span>What is Markdown?

Markdown is a plain text format for writing structured documents, based
on conventions for indicating formatting in email and usenet posts. It
was developed by John Gruber (with help from Aaron Swartz) and released
in 2004 in the form of a [syntax
description](http://daringfireball.net/projects/markdown/syntax) and a
Perl script (`Markdown.pl`) for converting Markdown to HTML. In the next
decade, dozens of implementations were developed in many languages. Some
extended the original Markdown syntax with conventions for footnotes,
tables, and other document elements. Some allowed Markdown documents to
be rendered in formats other than HTML. Websites like Reddit,
StackOverflow, and GitHub had millions of people using Markdown. And
Markdown started to be used beyond the web, to author books, articles,
slide shows, letters, and lecture notes.

What distinguishes Markdown from many other lightweight markup syntaxes,
which are often easier to write, is its readability. As Gruber writes:

> The overriding design goal for Markdown’s formatting syntax is to make
> it as readable as possible. The idea is that a Markdown-formatted
> document should be publishable as-is, as plain text, without looking
> like it’s been marked up with tags or formatting instructions.
> (<http://daringfireball.net/projects/markdown/>)

The point can be illustrated by comparing a sample of
[AsciiDoc](http://www.methods.co.nz/asciidoc/) with an equivalent sample
of Markdown. Here is a sample of AsciiDoc from the AsciiDoc manual:

    1. List item one.
          +
          List item one continued with a second paragraph followed by an
          Indented block.
          +
          .................
          $ ls *.sh
          $ mv *.sh ~/tmp
          .................
          +
          List item continued with a third paragraph.

          2. List item two continued with an open block.
          +
          --
          This paragraph is part of the preceding list item.

          a. This list is nested and does not require explicit item
          continuation.
          +
          This paragraph is part of the preceding list item.

          b. List item b.

          This paragraph belongs to item two of the outer list.
          --
        

And here is the equivalent in Markdown:

    1. List item one.

          List item one continued with a second paragraph followed by an
          Indented block.

          $ ls *.sh
          $ mv *.sh ~/tmp

          List item continued with a third paragraph.

          2. List item two continued with an open block.

          This paragraph is part of the preceding list item.

          1. This list is nested and does not require explicit item continuation.

          This paragraph is part of the preceding list item.

          2. List item b.

          This paragraph belongs to item two of the outer list.
        

The AsciiDoc version is, arguably, easier to write. You don’t need to
worry about indentation. But the Markdown version is much easier to
read. The nesting of list items is apparent to the eye in the source,
not just in the processed document.

## <a href="#TOC" class="toc-link"></a><span class="number">1.3</span>Why is a spec needed?

John Gruber’s [canonical description of Markdown’s
syntax](http://daringfireball.net/projects/markdown/syntax) does not
specify the syntax unambiguously. Here are some examples of questions it
does not answer:

1.  How much indentation is needed for a sublist? The spec says that
    continuation paragraphs need to be indented four spaces, but is not
    fully explicit about sublists. It is natural to think that they,
    too, must be indented four spaces, but `Markdown.pl` does not
    require that. This is hardly a “corner case,” and divergences
    between implementations on this issue often lead to surprises for
    users in real documents. (See [this comment by John
    Gruber](http://article.gmane.org/gmane.text.markdown.general/1997).)

2.  Is a blank line needed before a block quote or heading? Most
    implementations do not require the blank line. However, this can
    lead to unexpected results in hard-wrapped text, and also to
    ambiguities in parsing (note that some implementations put the
    heading inside the blockquote, while others do not). (John Gruber
    has also spoken [in favor of requiring the blank
    lines](http://article.gmane.org/gmane.text.markdown.general/2146).)

3.  Is a blank line needed before an indented code block? (`Markdown.pl`
    requires it, but this is not mentioned in the documentation, and
    some implementations do not require it.)

        paragraph
                  code?
                

4.  What is the exact rule for determining when list items get wrapped
    in `<p>` tags? Can a list be partially “loose” and partially
    “tight”? What should we do with a list like this?

        1. one

                  2. two
                  3. three
                

    Or this?

        1. one
                  - a

                  - b
                  2. two
                

    (There are some relevant comments by John Gruber
    [here](http://article.gmane.org/gmane.text.markdown.general/2554).)

5.  Can list markers be indented? Can ordered list markers be
    right-aligned?

         8. item 1
                  9. item 2
                  10. item 2a
                

6.  Is this one list with a thematic break in its second item, or two
    lists separated by a thematic break?

        * a
                  * * * * *
                  * b
                

7.  When list markers change from numbers to bullets, do we have two
    lists or one? (The Markdown syntax description suggests two, but the
    perl scripts and many other implementations produce one.)

        1. fee
                  2. fie
                  - foe
                  - fum
                

8.  What are the precedence rules for the markers of inline structure?
    For example, is the following a valid link, or does the code span
    take precedence ?

        [a backtick (`)](/url) and [another backtick (`)](/url).
                

9.  What are the precedence rules for markers of emphasis and strong
    emphasis? For example, how should the following be parsed?

        *foo *bar* baz*
                

10. What are the precedence rules between block-level and inline-level
    structure? For example, how should the following be parsed?

        - `a long code span can contain a hyphen like this
                  - and it can screw things up`
                

11. Can list items include section headings? (`Markdown.pl` does not
    allow this, but does allow blockquotes to include headings.)

        - # Heading
                

12. Can list items be empty?

        * a
                  *
                  * b
                

13. Can link references be defined inside block quotes or list items?

        > Blockquote [foo].
                  >
                  > [foo]: /url
                

14. If there are multiple definitions for the same reference, which
    takes precedence?

        [foo]: /url1
                  [foo]: /url2

                  [foo][]
                

In the absence of a spec, early implementers consulted `Markdown.pl` to
resolve these ambiguities. But `Markdown.pl` was quite buggy, and gave
manifestly bad results in many cases, so it was not a satisfactory
replacement for a spec.

Because there is no unambiguous spec, implementations have diverged
considerably. As a result, users are often surprised to find that a
document that renders one way on one system (say, a GitHub wiki) renders
differently on another (say, converting to docbook using pandoc). To
make matters worse, because nothing in Markdown counts as a “syntax
error,” the divergence often isn’t discovered right away.

## <a href="#TOC" class="toc-link"></a><span class="number">1.4</span>About this document

This document attempts to specify Markdown syntax unambiguously. It
contains many examples with side-by-side Markdown and HTML. These are
intended to double as conformance tests. An accompanying script
`spec_tests.py` can be used to run the tests against any Markdown
program:

    python test/spec_tests.py --spec spec.txt --program PROGRAM
        

Since this document describes how Markdown is to be parsed into an
abstract syntax tree, it would have made sense to use an abstract
representation of the syntax tree instead of HTML. But HTML is capable
of representing the structural distinctions we need to make, and the
choice of HTML for the tests makes it possible to run the tests against
an implementation without writing an abstract syntax tree renderer.

This document is generated from a text file, `spec.txt`, written in
Markdown with a small extension for the side-by-side tests. The script
`tools/makespec.py` can be used to convert `spec.txt` into HTML or
CommonMark (which can then be converted into other formats).

In the examples, the `→` character is used to represent tabs.

# <span class="number">2</span>Preliminaries

## <a href="#TOC" class="toc-link"></a><span class="number">2.1</span>Characters and lines

Any sequence of [characters](#character) is a valid CommonMark document.

A <a href="#character" id="character" class="definition">character</a>
is a Unicode code point. Although some code points (for example,
combining accents) do not correspond to characters in an intuitive
sense, all code points count as characters for purposes of this spec.

This spec does not specify an encoding; it thinks of lines as composed
of [characters](#character) rather than bytes. A conforming parser may
be limited to a certain encoding.

A <a href="#line" id="line" class="definition">line</a> is a sequence of
zero or more [characters](#character) other than newline (`U+000A`) or
carriage return (`U+000D`), followed by a [line ending](#line-ending) or
by the end of file.

A <a href="#line-ending" id="line-ending" class="definition">line
ending</a> is a newline (`U+000A`), a carriage return (`U+000D`) not
followed by a newline, or a carriage return and a following newline.

A line containing no characters, or a line containing only spaces
(`U+0020`) or tabs (`U+0009`), is called a
<a href="#blank-line" id="blank-line" class="definition">blank line</a>.

The following definitions of character classes will be used in this
spec:

A <a href="#whitespace-character" id="whitespace-character"
class="definition">whitespace character</a> is a space (`U+0020`), tab
(`U+0009`), newline (`U+000A`), line tabulation (`U+000B`), form feed
(`U+000C`), or carriage return (`U+000D`).

<a href="#whitespace" id="whitespace" class="definition">Whitespace</a>
is a sequence of one or more [whitespace
characters](#whitespace-character).

A <a href="#unicode-whitespace-character"
id="unicode-whitespace-character" class="definition">Unicode whitespace
character</a> is any code point in the Unicode `Zs` general category, or
a tab (`U+0009`), carriage return (`U+000D`), newline (`U+000A`), or
form feed (`U+000C`).

<a href="#unicode-whitespace" id="unicode-whitespace"
class="definition">Unicode whitespace</a> is a sequence of one or more
[Unicode whitespace characters](#unicode-whitespace-character).

A <a href="#space" id="space" class="definition">space</a> is `U+0020`.

A <a href="#non-whitespace-character" id="non-whitespace-character"
class="definition">non-whitespace character</a> is any character that is
not a [whitespace character](#whitespace-character).

An
<a href="#ascii-punctuation-character" id="ascii-punctuation-character"
class="definition">ASCII punctuation character</a> is `!`, `"`, `#`,
`$`, `%`, `&`, `'`, `(`, `)`, `*`, `+`, `,`, `-`, `.`, `/` (U+0021–2F),
`:`, `;`, `<`, `=`, `>`, `?`, `@` (U+003A–0040), `[`, `\`, `]`, `^`,
`_`, `` ` `` (U+005B–0060), `{`, `|`, `}`, or `~` (U+007B–007E).

A <a href="#punctuation-character" id="punctuation-character"
class="definition">punctuation character</a> is an [ASCII punctuation
character](#ascii-punctuation-character) or anything in the general
Unicode categories `Pc`, `Pd`, `Pe`, `Pf`, `Pi`, `Po`, or `Ps`.

## <a href="#TOC" class="toc-link"></a><span class="number">2.2</span>Tabs

Tabs in lines are not expanded to [spaces](#space). However, in contexts
where whitespace helps to define block structure, tabs behave as if they
were replaced by spaces with a tab stop of 4 characters.

Thus, for example, a tab can be used instead of four spaces in an
indented code block. (Note, however, that internal tabs are passed
through as literal tabs, not expanded to spaces.)

[Example 1](#example-1)

    →foo→baz→→bim
            

    <pre><code>foo→baz→→bim
              </code></pre>
            

[Example 2](#example-2)

      →foo→baz→→bim
            

    <pre><code>foo→baz→→bim
              </code></pre>
            

[Example 3](#example-3)

      
               a→a
                 
              ὐ→a
            

    <pre><code>a→a
              ὐ→a
              </code></pre>
            

In the following example, a continuation paragraph of a list item is
indented with a tab; this has exactly the same effect as indentation
with four spaces would:

[Example 4](#example-4)

      -
              foo

              →bar
            

    <ul>
              <li>
              <p>foo</p>
              <p>bar</p>
              </li>
              </ul>
            

[Example 5](#example-5)

    - foo

              →→bar
            

    <ul>
              <li>
              <p>foo</p>
              <pre><code>  bar
              </code></pre>
              </li>
              </ul>
            

Normally the `>` that begins a block quote may be followed optionally by
a space, which is not considered part of the content. In the following
case `>` is followed by a tab, which is treated as if it were expanded
into three spaces. Since one of these spaces is considered part of the
delimiter, `foo` is considered to be indented six spaces inside the
block quote context, so we get an indented code block starting with two
spaces.

[Example 6](#example-6)

    >→→foo
            

    <blockquote>
              <pre><code>  foo
              </code></pre>
              </blockquote>
            

[Example 7](#example-7)

    -→→foo
            

    <ul>
              <li>
              <pre><code>  foo
              </code></pre>
              </li>
              </ul>
            

[Example 8](#example-8)

      
               foo
              →bar
            

    <pre><code>foo
              bar
              </code></pre>
            

[Example 9](#example-9)

     - foo
                 -
              bar
              → - baz
            

    <ul>
              <li>foo
              <ul>
              <li>bar
              <ul>
              <li>baz</li>
              </ul>
              </li>
              </ul>
              </li>
              </ul>
            

[Example 10](#example-10)

    #→Foo
            

    <h1>Foo</h1>
            

[Example 11](#example-11)

    *→*→*→
            

    <hr />
            

## <a href="#TOC" class="toc-link"></a><span class="number">2.3</span>Insecure characters

For security reasons, the Unicode character `U+0000` must be replaced
with the REPLACEMENT CHARACTER (`U+FFFD`).

# <span class="number">3</span>Blocks and inlines

We can think of a document as a sequence of
<a href="#blocks" id="blocks" class="definition">blocks</a>—structural
elements like paragraphs, block quotations, lists, headings, rules, and
code blocks. Some blocks (like block quotes and list items) contain
other blocks; others (like headings and paragraphs) contain
<a href="#inline" id="inline" class="definition">inline</a>
content—text, links, emphasized text, images, code spans, and so on.

## <a href="#TOC" class="toc-link"></a><span class="number">3.1</span>Precedence

Indicators of block structure always take precedence over indicators of
inline structure. So, for example, the following is a list with two
items, not a list with one item containing a code span:

[Example 12](#example-12)

    - `one
              - two`
            

    <ul>
              <li>`one</li>
              <li>two`</li>
              </ul>
            

This means that parsing can proceed in two steps: first, the block
structure of the document can be discerned; second, text lines inside
paragraphs, headings, and other block constructs can be parsed for
inline structure. The second step requires information about link
reference definitions that will be available only at the end of the
first step. Note that the first step requires processing lines in
sequence, but the second can be parallelized, since the inline parsing
of one block element does not affect the inline parsing of any other.

## <a href="#TOC" class="toc-link"></a><span class="number">3.2</span>Container blocks and leaf blocks

We can divide blocks into two types:
<a href="#container-blocks" id="container-blocks"
class="definition">container blocks</a>, which can contain other blocks,
and <a href="#leaf-blocks" id="leaf-blocks" class="definition">leaf
blocks</a>, which cannot.

# <span class="number">4</span>Leaf blocks

This section describes the different kinds of leaf block that make up a
Markdown document.

## <a href="#TOC" class="toc-link"></a><span class="number">4.1</span>Thematic breaks

A line consisting of 0-3 spaces of indentation, followed by a sequence
of three or more matching `-`, `_`, or `*` characters, each followed
optionally by any number of spaces or tabs, forms a
<a href="#thematic-break" id="thematic-break"
class="definition">thematic break</a>.

[Example 13](#example-13)

    ***
              ---
              ___
            

    <hr />
              <hr />
              <hr />
            

Wrong characters:

[Example 14](#example-14)

    +++
            

    <p>+++</p>
            

[Example 15](#example-15)

    ===
            

    <p>===</p>
            

Not enough characters:

[Example 16](#example-16)

    --
              **
              __
            

    <p>--
              **
              __</p>
            

One to three spaces indent are allowed:

[Example 17](#example-17)

     ***
                ***
                 ***
            

    <hr />
              <hr />
              <hr />
            

Four spaces is too many:

[Example 18](#example-18)

      
               ***
            

    <pre><code>***
              </code></pre>
            

[Example 19](#example-19)

    Foo
                 
              ***
            

    <p>Foo
              ***</p>
            

More than three characters may be used:

[Example 20](#example-20)

    _____________________________________
            

    <hr />
            

Spaces are allowed between the characters:

[Example 21](#example-21)

     - - -
            

    <hr />
            

[Example 22](#example-22)

     **  * ** *
              ** * **
            

    <hr />
            

[Example 23](#example-23)

    -  
                -    
               -      -
            

    <hr />
            

Spaces are allowed at the end:

[Example 24](#example-24)

    - - - -  
               
            

    <hr />
            

However, no other characters may occur in the line:

[Example 25](#example-25)

    _ _ _ _ a

              a------

              ---a---
            

    <p>_ _ _ _ a</p>
              <p>a------</p>
              <p>---a---</p>
            

It is required that all of the [non-whitespace
characters](#non-whitespace-character) be the same. So, this is not a
thematic break:

[Example 26](#example-26)

     *-*
            

    <p><em>-</em></p>
            

Thematic breaks do not need blank lines before or after:

[Example 27](#example-27)

    - foo
              ***
              - bar
            

    <ul>
              <li>foo</li>
              </ul>
              <hr />
              <ul>
              <li>bar</li>
              </ul>
            

Thematic breaks can interrupt a paragraph:

[Example 28](#example-28)

    Foo
              ***
              bar
            

    <p>Foo</p>
              <hr />
              <p>bar</p>
            

If a line of dashes that meets the above conditions for being a thematic
break could also be interpreted as the underline of a [setext
heading](#setext-heading), the interpretation as a [setext
heading](#setext-heading) takes precedence. Thus, for example, this is a
setext heading, not a paragraph followed by a thematic break:

[Example 29](#example-29)

    Foo
              ---
              bar
            

    <h2>Foo</h2>
              <p>bar</p>
            

When both a thematic break and a list item are possible interpretations
of a line, the thematic break takes precedence:

[Example 30](#example-30)

    * Foo
              * * *
              * Bar
            

    <ul>
              <li>Foo</li>
              </ul>
              <hr />
              <ul>
              <li>Bar</li>
              </ul>
            

If you want a thematic break in a list item, use a different bullet:

[Example 31](#example-31)

    - Foo
              - * * *
            

    <ul>
              <li>Foo</li>
              <li>
              <hr />
              </li>
              </ul>
            

## <a href="#TOC" class="toc-link"></a><span class="number">4.2</span>ATX headings

An <a href="#atx-heading" id="atx-heading" class="definition">ATX
heading</a> consists of a string of characters, parsed as inline
content, between an opening sequence of 1–6 unescaped `#` characters and
an optional closing sequence of any number of unescaped `#` characters.
The opening sequence of `#` characters must be followed by a
[space](#space) or by the end of line. The optional closing sequence of
`#`s must be preceded by a [space](#space) and may be followed by spaces
only. The opening `#` character may be indented 0-3 spaces. The raw
contents of the heading are stripped of leading and trailing spaces
before being parsed as inline content. The heading level is equal to the
number of `#` characters in the opening sequence.

Simple headings:

[Example 32](#example-32)

    # foo
              ## foo
              ### foo
              #### foo
              ##### foo
              ###### foo
            

    <h1>foo</h1>
              <h2>foo</h2>
              <h3>foo</h3>
              <h4>foo</h4>
              <h5>foo</h5>
              <h6>foo</h6>
            

More than six `#` characters is not a heading:

[Example 33](#example-33)

    ####### foo
            

    <p>####### foo</p>
            

At least one space is required between the `#` characters and the
heading’s contents, unless the heading is empty. Note that many
implementations currently do not require the space. However, the space
was required by the [original ATX
implementation](http://www.aaronsw.com/2002/atx/atx.py), and it helps
prevent things like the following from being parsed as headings:

[Example 34](#example-34)

    #5 bolt

              #hashtag
            

    <p>#5 bolt</p>
              <p>#hashtag</p>
            

This is not a heading, because the first `#` is escaped:

[Example 35](#example-35)

    \## foo
            

    <p>## foo</p>
            

Contents are parsed as inlines:

[Example 36](#example-36)

    # foo *bar* \*baz\*
            

    <h1>foo <em>bar</em> *baz*</h1>
            

Leading and trailing [whitespace](#whitespace) is ignored in parsing
inline content:

[Example 37](#example-37)

    #  
                    
                    
               foo     
                    
                    
               
            

    <h1>foo</h1>
            

One to three spaces indentation are allowed:

[Example 38](#example-38)

     ### foo
                ## foo
                 #
              foo
            

    <h3>foo</h3>
              <h2>foo</h2>
              <h1>foo</h1>
            

Four spaces are too much:

[Example 39](#example-39)

      
               # foo
            

    <pre><code># foo
              </code></pre>
            

[Example 40](#example-40)

    foo
                 
              # bar
            

    <p>foo
              # bar</p>
            

A closing sequence of `#` characters is optional:

[Example 41](#example-41)

    ## foo ##
                ### 
               bar    ###
            

    <h2>foo</h2>
              <h3>bar</h3>
            

It need not be the same length as the opening sequence:

[Example 42](#example-42)

    # foo
              ##################################
              ##### foo ##
            

    <h1>foo</h1>
              <h5>foo</h5>
            

Spaces are allowed after the closing sequence:

[Example 43](#example-43)

    ### foo ###   
               
            

    <h3>foo</h3>
            

A sequence of `#` characters with anything but [spaces](#space)
following it is not a closing sequence, but counts as part of the
contents of the heading:

[Example 44](#example-44)

    ### foo ### b
            

    <h3>foo ###
              b</h3>
            

The closing sequence must be preceded by a space:

[Example 45](#example-45)

    # foo#
            

    <h1>foo#</h1>
            

Backslash-escaped `#` characters do not count as part of the closing
sequence:

[Example 46](#example-46)

    ### foo \###
              ## foo #\##
              # foo \#
            

    <h3>foo ###</h3>
              <h2>foo ###</h2>
              <h1>foo #</h1>
            

ATX headings need not be separated from surrounding content by blank
lines, and they can interrupt paragraphs:

[Example 47](#example-47)

    ****
              ## foo
              ****
            

    <hr />
              <h2>foo</h2>
              <hr />
            

[Example 48](#example-48)

    Foo bar
              # baz
              Bar foo
            

    <p>Foo bar</p>
              <h1>baz</h1>
              <p>Bar foo</p>
            

ATX headings can be empty:

[Example 49](#example-49)

    ## 
              #
              ### ###
            

    <h2></h2>
              <h1></h1>
              <h3></h3>
            

## <a href="#TOC" class="toc-link"></a><span class="number">4.3</span>Setext headings

A
<a href="#setext-heading" id="setext-heading" class="definition">setext
heading</a> consists of one or more lines of text, each containing at
least one [non-whitespace character](#non-whitespace-character), with no
more than 3 spaces indentation, followed by a [setext heading
underline](#setext-heading-underline). The lines of text must be such
that, were they not followed by the setext heading underline, they would
be interpreted as a paragraph: they cannot be interpretable as a [code
fence](#code-fence), [ATX heading](#atx-headings), [block
quote](#block-quotes), [thematic break](#thematic-break), [list
item](#list-items), or [HTML block](#html-blocks).

A <a href="#setext-heading-underline" id="setext-heading-underline"
class="definition">setext heading underline</a> is a sequence of `=`
characters or a sequence of `-` characters, with no more than 3 spaces
indentation and any number of trailing spaces. If a line containing a
single `-` can be interpreted as an empty [list items](#list-items), it
should be interpreted this way and not as a [setext heading
underline](#setext-heading-underline).

The heading is a level 1 heading if `=` characters are used in the
[setext heading underline](#setext-heading-underline), and a level 2
heading if `-` characters are used. The contents of the heading are the
result of parsing the preceding lines of text as CommonMark inline
content.

In general, a setext heading need not be preceded or followed by a blank
line. However, it cannot interrupt a paragraph, so when a setext heading
comes after a paragraph, a blank line is needed between them.

Simple examples:

[Example 50](#example-50)

    Foo *bar*
              =========

              Foo *bar*
              ---------
            

    <h1>Foo <em>bar</em></h1>
              <h2>Foo <em>bar</em></h2>
            

The content of the header may span more than one line:

[Example 51](#example-51)

    Foo *bar
              baz*
              ====
            

    <h1>Foo <em>bar
              baz</em></h1>
            

The contents are the result of parsing the headings’s raw content as
inlines. The heading’s raw content is formed by concatenating the lines
and removing initial and final [whitespace](#whitespace).

[Example 52](#example-52)

      Foo *bar
              baz*→
              ====
            

    <h1>Foo <em>bar
              baz</em></h1>
            

The underlining can be any length:

[Example 53](#example-53)

    Foo
              -------------------------

              Foo
              =
            

    <h2>Foo</h2>
              <h1>Foo</h1>
            

The heading content can be indented up to three spaces, and need not
line up with the underlining:

[Example 54](#example-54)

      
              Foo
              ---

                Foo
              -----

                Foo
                ===
            

    <h2>Foo</h2>
              <h2>Foo</h2>
              <h1>Foo</h1>
            

Four spaces indent is too much:

[Example 55](#example-55)

      
               Foo
                 
              ---

                 
              Foo
              ---
            

    <pre><code>Foo
              ---

              Foo
              </code></pre>
              <hr />
            

The setext heading underline can be indented up to three spaces, and may
have trailing spaces:

[Example 56](#example-56)

    Foo
                 ----
                   
            

    <h2>Foo</h2>
            

Four spaces is too much:

[Example 57](#example-57)

    Foo
                 
              ---
            

    <p>Foo
              ---</p>
            

The setext heading underline cannot contain internal spaces:

[Example 58](#example-58)

    Foo
              = =

              Foo
              --- -
            

    <p>Foo
              = =</p>
              <p>Foo</p>
              <hr />
            

Trailing spaces in the content line do not cause a line break:

[Example 59](#example-59)

    Foo  
              -----
            

    <h2>Foo</h2>
            

Nor does a backslash at the end:

[Example 60](#example-60)

    Foo\
              ----
            

    <h2>Foo\</h2>
            

Since indicators of block structure take precedence over indicators of
inline structure, the following are setext headings:

[Example 61](#example-61)

    `Foo
              ----
              `

              <a title="a lot
              ---
              of dashes"/>
            

    <h2>`Foo</h2>
              <p>`</p>
              <h2>&lt;a title=&quot;a lot</h2>
              <p>of dashes&quot;/&gt;</p>
            

The setext heading underline cannot be a [lazy continuation
line](#lazy-continuation-line) in a list item or block quote:

[Example 62](#example-62)

    > Foo
              ---
            

    <blockquote>
              <p>Foo</p>
              </blockquote>
              <hr />
            

[Example 63](#example-63)

    > foo
              bar
              ===
            

    <blockquote>
              <p>foo
              bar
              ===</p>
              </blockquote>
            

[Example 64](#example-64)

    - Foo
              ---
            

    <ul>
              <li>Foo</li>
              </ul>
              <hr />
            

A blank line is needed between a paragraph and a following setext
heading, since otherwise the paragraph becomes part of the heading’s
content:

[Example 65](#example-65)

    Foo
              Bar
              ---
            

    <h2>Foo
              Bar</h2>
            

But in general a blank line is not required before or after setext
headings:

[Example 66](#example-66)

    ---
              Foo
              ---
              Bar
              ---
              Baz
            

    <hr />
              <h2>Foo</h2>
              <h2>Bar</h2>
              <p>Baz</p>
            

Setext headings cannot be empty:

[Example 67](#example-67)


              ====
            

    <p>====</p>
            

Setext heading text lines must not be interpretable as block constructs
other than paragraphs. So, the line of dashes in these examples gets
interpreted as a thematic break:

[Example 68](#example-68)

    ---
              ---
            

    <hr />
              <hr />
            

[Example 69](#example-69)

    - foo
              -----
            

    <ul>
              <li>foo</li>
              </ul>
              <hr />
            

[Example 70](#example-70)

      
               foo
              ---
            

    <pre><code>foo
              </code></pre>
              <hr />
            

[Example 71](#example-71)

    > foo
              -----
            

    <blockquote>
              <p>foo</p>
              </blockquote>
              <hr />
            

If you want a heading with `> foo` as its literal text, you can use
backslash escapes:

[Example 72](#example-72)

    \> foo
              ------
            

    <h2>&gt; foo</h2>
            

**Compatibility note:** Most existing Markdown implementations do not
allow the text of setext headings to span multiple lines. But there is
no consensus about how to interpret

    Foo
          bar
          ---
          baz
        

One can find four different interpretations:

1.  paragraph “Foo”, heading “bar”, paragraph “baz”
2.  paragraph “Foo bar”, thematic break, paragraph “baz”
3.  paragraph “Foo bar — baz”
4.  heading “Foo bar”, paragraph “baz”

We find interpretation 4 most natural, and interpretation 4 increases
the expressive power of CommonMark, by allowing multiline headings.
Authors who want interpretation 1 can put a blank line after the first
paragraph:

[Example 73](#example-73)

    Foo

              bar
              ---
              baz
            

    <p>Foo</p>
              <h2>bar</h2>
              <p>baz</p>
            

Authors who want interpretation 2 can put blank lines around the
thematic break,

[Example 74](#example-74)

    Foo
              bar

              ---

              baz
            

    <p>Foo
              bar</p>
              <hr />
              <p>baz</p>
            

or use a thematic break that cannot count as a [setext heading
underline](#setext-heading-underline), such as

[Example 75](#example-75)

    Foo
              bar
              * * *
              baz
            

    <p>Foo
              bar</p>
              <hr />
              <p>baz</p>
            

Authors who want interpretation 3 can use backslash escapes:

[Example 76](#example-76)

    Foo
              bar
              \---
              baz
            

    <p>Foo
              bar
              ---
              baz</p>
            

## <a href="#TOC" class="toc-link"></a><span class="number">4.4</span>Indented code blocks

An <a href="#indented-code-block" id="indented-code-block"
class="definition">indented code block</a> is composed of one or more
[indented chunks](#indented-chunk) separated by blank lines. An
<a href="#indented-chunk" id="indented-chunk"
class="definition">indented chunk</a> is a sequence of non-blank lines,
each indented four or more spaces. The contents of the code block are
the literal contents of the lines, including trailing [line
endings](#line-ending), minus four spaces of indentation. An indented
code block has no [info string](#info-string).

An indented code block cannot interrupt a paragraph, so there must be a
blank line between a paragraph and a following indented code block. (A
blank line is not needed, however, between a code block and a following
paragraph.)

[Example 77](#example-77)

      
               a simple
                 
                indented code block
            

    <pre><code>a simple
                indented code block
              </code></pre>
            

If there is any ambiguity between an interpretation of indentation as a
code block and as indicating that material belongs to a [list
item](#list-items), the list item interpretation takes precedence:

[Example 78](#example-78)

      -
              foo

                 
              bar
            

    <ul>
              <li>
              <p>foo</p>
              <p>bar</p>
              </li>
              </ul>
            

[Example 79](#example-79)

    1.  foo

                 
              - bar
            

    <ol>
              <li>
              <p>foo</p>
              <ul>
              <li>bar</li>
              </ul>
              </li>
              </ol>
            

The contents of a code block are literal text, and do not get parsed as
Markdown:

[Example 80](#example-80)

      
               <a/>
                 
              *hi*

                 
              - one
            

    <pre><code>&lt;a/&gt;
              *hi*

              - one
              </code></pre>
            

Here we have three chunks separated by blank lines:

[Example 81](#example-81)

      
               chunk1

                 
              chunk2
                
               
               
                 
              chunk3
            

    <pre><code>chunk1

              chunk2



              chunk3
              </code></pre>
            

Any initial spaces beyond four will be included in the content, even in
interior blank lines:

[Example 82](#example-82)

      
               chunk1
                 
                
                 
                chunk2
            

    <pre><code>chunk1
                
                chunk2
              </code></pre>
            

An indented code block cannot interrupt a paragraph. (This allows
hanging indents and the like.)

[Example 83](#example-83)

    Foo
                 
              bar
            

    <p>Foo
              bar</p>
            

However, any non-blank line with fewer than four leading spaces ends the
code block immediately. So a paragraph may occur immediately after
indented code:

[Example 84](#example-84)

      
               foo
              bar
            

    <pre><code>foo
              </code></pre>
              <p>bar</p>
            

And indented code can occur immediately before and after other kinds of
blocks:

[Example 85](#example-85)

    # Heading
                 
              foo
              Heading
              ------
                 
              foo
              ----
            

    <h1>Heading</h1>
              <pre><code>foo
              </code></pre>
              <h2>Heading</h2>
              <pre><code>foo
              </code></pre>
              <hr />
            

The first line can be indented more than four spaces:

[Example 86](#example-86)

      
                   foo
                 
              bar
            

    <pre><code> 
                foo
              bar
              </code></pre>
            

Blank lines preceding or following an indented code block are not
included in it:

[Example 87](#example-87)


                 
              
                 
              foo
                 
              
            

    <pre><code>foo
              </code></pre>
            

Trailing spaces are included in the code block’s content:

[Example 88](#example-88)

      
               foo  
            

    <pre><code>foo 
              
              </code></pre>
            

## <a href="#TOC" class="toc-link"></a><span class="number">4.5</span>Fenced code blocks

A
<a href="#code-fence" id="code-fence" class="definition">code fence</a>
is a sequence of at least three consecutive backtick characters
(`` ` ``) or tildes (`~`). (Tildes and backticks cannot be mixed.) A
<a href="#fenced-code-block" id="fenced-code-block"
class="definition">fenced code block</a> begins with a code fence,
indented no more than three spaces.

The line with the opening code fence may optionally contain some text
following the code fence; this is trimmed of leading and trailing
whitespace and called the
<a href="#info-string" id="info-string" class="definition">info
string</a>. If the [info string](#info-string) comes after a backtick
fence, it may not contain any backtick characters. (The reason for this
restriction is that otherwise some inline code would be incorrectly
interpreted as the beginning of a fenced code block.)

The content of the code block consists of all subsequent lines, until a
closing [code fence](#code-fence) of the same type as the code block
began with (backticks or tildes), and with at least as many backticks or
tildes as the opening code fence. If the leading code fence is indented
N spaces, then up to N spaces of indentation are removed from each line
of the content (if present). (If a content line is not indented, it is
preserved unchanged. If it is indented less than N spaces, all of the
indentation is removed.)

The closing code fence may be indented up to three spaces, and may be
followed only by spaces, which are ignored. If the end of the containing
block (or document) is reached and no closing code fence has been found,
the code block contains all of the lines after the opening code fence
until the end of the containing block (or document). (An alternative
spec would require backtracking in the event that a closing code fence
is not found. But this makes parsing much less efficient, and there
seems to be no real down side to the behavior described here.)

A fenced code block may interrupt a paragraph, and does not require a
blank line either before or after.

The content of a code fence is treated as literal text, not parsed as
inlines. The first word of the [info string](#info-string) is typically
used to specify the language of the code sample, and rendered in the
`class` attribute of the `code` tag. However, this spec does not mandate
any particular treatment of the [info string](#info-string).

Here is a simple example with backticks:

[Example 89](#example-89)

    ```
              <
               >
              ```
            

    <pre><code>&lt;
               &gt;
              </code></pre>
            

With tildes:

[Example 90](#example-90)

    ~~~
              <
               >
              ~~~
            

    <pre><code>&lt;
               &gt;
              </code></pre>
            

Fewer than three backticks is not enough:

[Example 91](#example-91)

    ``
              foo
              ``
            

    <p><code>foo</code></p>
            

The closing code fence must use the same character as the opening fence:

[Example 92](#example-92)

    ```
              aaa
              ~~~
              ```
            

    <pre><code>aaa
              ~~~
              </code></pre>
            

[Example 93](#example-93)

    ~~~
              aaa
              ```
              ~~~
            

    <pre><code>aaa
              ```
              </code></pre>
            

The closing code fence must be at least as long as the opening fence:

[Example 94](#example-94)

    ````
              aaa
              ```
              ``````
            

    <pre><code>aaa
              ```
              </code></pre>
            

[Example 95](#example-95)

    ~~~~
              aaa
              ~~~
              ~~~~
            

    <pre><code>aaa
              ~~~
              </code></pre>
            

Unclosed code blocks are closed by the end of the document (or the
enclosing [block quote](#block-quotes) or [list item](#list-items)):

[Example 96](#example-96)

    ```
            

    <pre><code></code></pre>
            

[Example 97](#example-97)

    `````

              ```
              aaa
            

    <pre><code>
              ```
              aaa
              </code></pre>
            

[Example 98](#example-98)

    > ```
              > aaa

              bbb
            

    <blockquote>
              <pre><code>aaa
              </code></pre>
              </blockquote>
              <p>bbb</p>
            

A code block can have all empty lines as its content:

[Example 99](#example-99)

    ```

                
              ```
            

    <pre><code>
                
              </code></pre>
            

A code block can be empty:

[Example 100](#example-100)

    ```
              ```
            

    <pre><code></code></pre>
            

Fences can be indented. If the opening fence is indented, content lines
will have equivalent opening indentation removed, if present:

[Example 101](#example-101)

     ```
               aaa
              aaa
              ```
            

    <pre><code>aaa
              aaa
              </code></pre>
            

[Example 102](#example-102)

      ```
              aaa
                aaa
              aaa
                ```
            

    <pre><code>aaa
              aaa
              aaa
              </code></pre>
            

[Example 103](#example-103)

      
              ```
                 aaa
                 
              aaa
                aaa
                 ```
            

    <pre><code>aaa
               aaa
              aaa
              </code></pre>
            

Four spaces indentation produces an indented code block:

[Example 104](#example-104)

      
               ```
                 
              aaa
                 
              ```
            

    <pre><code>```
              aaa
              ```
              </code></pre>
            

Closing fences may be indented by 0-3 spaces, and their indentation need
not match that of the opening fence:

[Example 105](#example-105)

    ```
              aaa
                ```
            

    <pre><code>aaa
              </code></pre>
            

[Example 106](#example-106)

      
              ```
              aaa
                ```
            

    <pre><code>aaa
              </code></pre>
            

This is not a closing fence, because it is indented 4 spaces:

[Example 107](#example-107)

    ```
              aaa
                 
              ```
            

    <pre><code>aaa
                 
              ```
              </code></pre>
            

Code fences (opening and closing) cannot contain internal spaces:

[Example 108](#example-108)

    ``` ```
              aaa
            

    <p><code> </code>
              aaa</p>
            

[Example 109](#example-109)

    ~~~~~~
              aaa
              ~~~ ~~
            

    <pre><code>aaa
              ~~~ ~~
              </code></pre>
            

Fenced code blocks can interrupt paragraphs, and can be followed
directly by paragraphs, without a blank line between:

[Example 110](#example-110)

    foo
              ```
              bar
              ```
              baz
            

    <p>foo</p>
              <pre><code>bar
              </code></pre>
              <p>baz</p>
            

Other blocks can also occur before and after fenced code blocks without
an intervening blank line:

[Example 111](#example-111)

    foo
              ---
              ~~~
              bar
              ~~~
              # baz
            

    <h2>foo</h2>
              <pre><code>bar
              </code></pre>
              <h1>baz</h1>
            

An [info string](#info-string) can be provided after the opening code
fence. Although this spec doesn’t mandate any particular treatment of
the info string, the first word is typically used to specify the
language of the code block. In HTML output, the language is normally
indicated by adding a class to the `code` element consisting of
`language-` followed by the language name.

[Example 112](#example-112)

    ```ruby
              def foo(x)
                return 3
              end
              ```
            

    <pre><code
              class="language-ruby">def foo(x)
                return 3
              end
              </code></pre>
            

[Example 113](#example-113)

    ~~~~    ruby startline=3 $%@#$
              def foo(x)
                return 3
              end
              ~~~~~~~
            

    <pre><code
              class="language-ruby">def foo(x)
                return 3
              end
              </code></pre>
            

[Example 114](#example-114)

    ````;
              ````
            

    <pre><code
              class="language-;"></code></pre>
            

[Info strings](#info-string) for backtick code blocks cannot contain
backticks:

[Example 115](#example-115)

    ``` aa ```
              foo
            

    <p><code>aa</code>
              foo</p>
            

[Info strings](#info-string) for tilde code blocks can contain backticks
and tildes:

[Example 116](#example-116)

    ~~~ aa ``` ~~~
              foo
              ~~~
            

    <pre><code
              class="language-aa">foo
              </code></pre>
            

Closing code fences cannot have [info strings](#info-string):

[Example 117](#example-117)

    ```
              ``` aaa
              ```
            

    <pre><code>``` aaa
              </code></pre>
            

## <a href="#TOC" class="toc-link"></a><span class="number">4.6</span>HTML blocks

An
<a href="#html-block" id="html-block" class="definition">HTML block</a>
is a group of lines that is treated as raw HTML (and will not be escaped
in HTML output).

There are seven kinds of [HTML block](#html-block), which can be defined
by their start and end conditions. The block begins with a line that
meets a
<a href="#start-condition" id="start-condition" class="definition">start
condition</a> (after up to three spaces optional indentation). It ends
with the first subsequent line that meets a matching
<a href="#end-condition" id="end-condition" class="definition">end
condition</a>, or the last line of the document, or the last line of the
[container block](#container-blocks) containing the current HTML block,
if no line is encountered that meets the [end
condition](#end-condition). If the first line meets both the [start
condition](#start-condition) and the [end condition](#end-condition),
the block will contain just that line.

1.  **Start condition:** line begins with the string `<script`, `<pre`,
    or `<style` (case-insensitive), followed by whitespace, the string
    `>`, or the end of the line.  
    **End condition:** line contains an end tag `</script>`, `</pre>`,
    or `</style>` (case-insensitive; it need not match the start tag).

2.  **Start condition:** line begins with the string `<!--`.  
    **End condition:** line contains the string `-->`.

3.  **Start condition:** line begins with the string `<?`.  
    **End condition:** line contains the string `?>`.

4.  **Start condition:** line begins with the string `<!` followed by an
    uppercase ASCII letter.  
    **End condition:** line contains the character `>`.

5.  **Start condition:** line begins with the string `<![CDATA[`.  
    **End condition:** line contains the string `]]>`.

6.  **Start condition:** line begins the string `<` or `</` followed by
    one of the strings (case-insensitive) `address`, `article`, `aside`,
    `base`, `basefont`, `blockquote`, `body`, `caption`, `center`,
    `col`, `colgroup`, `dd`, `details`, `dialog`, `dir`, `div`, `dl`,
    `dt`, `fieldset`, `figcaption`, `figure`, `footer`, `form`, `frame`,
    `frameset`, `h1`, `h2`, `h3`, `h4`, `h5`, `h6`, `head`, `header`,
    `hr`, `html`, `iframe`, `legend`, `li`, `link`, `main`, `menu`,
    `menuitem`, `nav`, `noframes`, `ol`, `optgroup`, `option`, `p`,
    `param`, `section`, `source`, `summary`, `table`, `tbody`, `td`,
    `tfoot`, `th`, `thead`, `title`, `tr`, `track`, `ul`, followed by
    [whitespace](#whitespace), the end of the line, the string `>`, or
    the string `/>`.  
    **End condition:** line is followed by a [blank line](#blank-line).

7.  **Start condition:** line begins with a complete [open
    tag](#open-tag) (with any [tag name](#tag-name) other than `script`,
    `style`, or `pre`) or a complete [closing tag](#closing-tag),
    followed only by [whitespace](#whitespace) or the end of the line.  
    **End condition:** line is followed by a [blank line](#blank-line).

HTML blocks continue until they are closed by their appropriate [end
condition](#end-condition), or the last line of the document or other
[container block](#container-blocks). This means any HTML **within an
HTML block** that might otherwise be recognised as a start condition
will be ignored by the parser and passed through as-is, without changing
the parser’s state.

For instance, `<pre>` within a HTML block started by `<table>` will not
affect the parser state; as the HTML block was started in by start
condition 6, it will end at any blank line. This can be surprising:

[Example 118](#example-118)

    <table><tr><td>
              <pre>
              **Hello**,

              _world_.
              </pre>
              </td></tr></table>
            

    <table><tr><td>
              <pre>
              **Hello**,
              <p><em>world</em>.
              </pre></p>
              </td></tr></table>
            

In this case, the HTML block is terminated by the newline — the
`**Hello**` text remains verbatim — and regular parsing resumes, with a
paragraph, emphasised `world` and inline and block HTML following.

All types of [HTML blocks](#html-blocks) except type 7 may interrupt a
paragraph. Blocks of type 7 may not interrupt a paragraph. (This
restriction is intended to prevent unwanted interpretation of long tags
inside a wrapped paragraph as starting HTML blocks.)

Some simple examples follow. Here are some basic HTML blocks of type 6:

[Example 119](#example-119)

    <table>
                <tr>
                 
              <td>
                 
                     hi
                 
              </td>
                </tr>
              </table>

              okay.
            

    <table>
                <tr>
                 
              <td>
                 
                     hi
                 
              </td>
                </tr>
              </table>
              <p>okay.</p>
            

[Example 120](#example-120)

     <div>
                *hello*
                 
                   <foo><a>
            

     <div>
                *hello*
                 
                   <foo><a>
            

A block can also start with a closing tag:

[Example 121](#example-121)

    </div>
              *foo*
            

    </div>
              *foo*
            

Here we have two HTML blocks with a Markdown paragraph between them:

[Example 122](#example-122)

    <DIV CLASS="foo">

              *Markdown*

              </DIV>
            

    <DIV CLASS="foo">
              <p><em>Markdown</em></p>
              </DIV>
            

The tag on the first line can be partial, as long as it is split where
there would be whitespace:

[Example 123](#example-123)

    <div id="foo"
                class="bar">
              </div>
            

    <div id="foo"
                class="bar">
              </div>
            

[Example 124](#example-124)

    <div id="foo"
              class="bar
                baz">
              </div>
            

    <div id="foo"
              class="bar
                baz">
              </div>
            

An open tag need not be closed:

[Example 125](#example-125)

    <div>
              *foo*

              *bar*
            

    <div>
              *foo*
              <p><em>bar</em></p>
            

A partial tag need not even be completed (garbage in, garbage out):

[Example 126](#example-126)

    <div id="foo"
              *hi*
            

    <div id="foo"
              *hi*
            

[Example 127](#example-127)

    <div class
              foo
            

    <div class
              foo
            

The initial tag doesn’t even need to be a valid tag, as long as it
starts like one:

[Example 128](#example-128)

    <div *???-&&&-<---
              *foo*
            

    <div *???-&&&-<---
              *foo*
            

In type 6 blocks, the initial tag need not be on a line by itself:

[Example 129](#example-129)

    <div><a
              href="bar">*foo*</a></div>
            

    <div><a
              href="bar">*foo*</a></div>
            

[Example 130](#example-130)

    <table><tr><td>
              foo
              </td></tr></table>
            

    <table><tr><td>
              foo
              </td></tr></table>
            

Everything until the next blank line or end of document gets included in
the HTML block. So, in the following example, what looks like a Markdown
code block is actually part of the HTML block, which continues until a
blank line or the end of the document is reached:

[Example 131](#example-131)

    <div></div>
              ``` c
              int x = 33;
              ```
            

    <div></div>
              ``` c
              int x = 33;
              ```
            

To start an [HTML block](#html-block) with a tag that is *not* in the
list of block-level tags in (6), you must put the tag by itself on the
first line (and it must be complete):

[Example 132](#example-132)

    <a href="foo">
              *bar*
              </a>
            

    <a href="foo">
              *bar*
              </a>
            

In type 7 blocks, the [tag name](#tag-name) can be anything:

[Example 133](#example-133)

    <Warning>
              *bar*
              </Warning>
            

    <Warning>
              *bar*
              </Warning>
            

[Example 134](#example-134)

    <i class="foo">
              *bar*
              </i>
            

    <i class="foo">
              *bar*
              </i>
            

[Example 135](#example-135)

    </ins>
              *bar*
            

    </ins>
              *bar*
            

These rules are designed to allow us to work with tags that can function
as either block-level or inline-level tags. The `<del>` tag is a nice
example. We can surround content with `<del>` tags in three different
ways. In this case, we get a raw HTML block, because the `<del>` tag is
on a line by itself:

[Example 136](#example-136)

    <del>
              *foo*
              </del>
            

    <del>
              *foo*
              </del>
            

In this case, we get a raw HTML block that just includes the `<del>` tag
(because it ends with the following blank line). So the contents get
interpreted as CommonMark:

[Example 137](#example-137)

    <del>

              *foo*

              </del>
            

    <del>
              <p><em>foo</em></p>
              </del>
            

Finally, in this case, the `<del>` tags are interpreted as [raw
HTML](#raw-html) *inside* the CommonMark paragraph. (Because the tag is
not on a line by itself, we get inline HTML rather than an [HTML
block](#html-block).)

[Example 138](#example-138)

    <del>*foo*</del>
            

    <p><del><em>foo</em></del></p>
            

HTML tags designed to contain literal content (`script`, `style`,
`pre`), comments, processing instructions, and declarations are treated
somewhat differently. Instead of ending at the first blank line, these
blocks end at the first line containing a corresponding end tag. As a
result, these blocks can contain blank lines:

A pre tag (type 1):

[Example 139](#example-139)

    <pre
              language="haskell"><code>
              import Text.HTML.TagSoup

              main :: IO ()
              main = print $ parseTags tags
              </code></pre>
              okay
            

    <pre language="haskell"><code>
              import Text.HTML.TagSoup

              main :: IO ()
              main = print $ parseTags tags
              </code></pre>
              <p>okay</p>
            

A script tag (type 1):

[Example 140](#example-140)

    <script type="text/javascript">
              // JavaScript example

              document.getElementById("demo").innerHTML =
              "Hello JavaScript!";
              </script>
              okay
            

    <script type="text/javascript">
              // JavaScript example

              document.getElementById("demo").innerHTML =
              "Hello JavaScript!";
              </script>
              <p>okay</p>
            

A style tag (type 1):

[Example 141](#example-141)

    <style
                type="text/css">
              h1 {color:red;}

              p {color:blue;}
              </style>
              okay
            

    <style
                type="text/css">
              h1 {color:red;}

              p {color:blue;}
              </style>
              <p>okay</p>
            

If there is no matching end tag, the block will end at the end of the
document (or the enclosing [block quote](#block-quotes) or [list
item](#list-items)):

[Example 142](#example-142)

    <style
                type="text/css">

              foo
            

    <style
                type="text/css">

              foo
            

[Example 143](#example-143)

    > <div>
              > foo

              bar
            

    <blockquote>
              <div>
              foo
              </blockquote>
              <p>bar</p>
            

[Example 144](#example-144)

    - <div>
              - foo
            

    <ul>
              <li>
              <div>
              </li>
              <li>foo</li>
              </ul>
            

The end tag can occur on the same line as the start tag:

[Example 145](#example-145)

    <style>p{color:red;}</style>
              *foo*
            

    <style>p{color:red;}</style>
              <p><em>foo</em></p>
            

[Example 146](#example-146)

    <!-- foo -->*bar*
              *baz*
            

    <!-- foo -->*bar*
              <p><em>baz</em></p>
            

Note that anything on the last line after the end tag will be included
in the [HTML block](#html-block):

[Example 147](#example-147)

    <script>
              foo
              </script>1. *bar*
            

    <script>
              foo
              </script>1. *bar*
            

A comment (type 2):

[Example 148](#example-148)

    <!-- Foo

              bar
                 baz
              -->
              okay
            

    <!-- Foo

              bar
                 baz
              -->
              <p>okay</p>
            

A processing instruction (type 3):

[Example 149](#example-149)

    <?php

                echo '>';

              ?>
              okay
            

    <?php

                echo '>';

              ?>
              <p>okay</p>
            

A declaration (type 4):

[Example 150](#example-150)

    <!DOCTYPE html>
            

    <!DOCTYPE html>
            

CDATA (type 5):

[Example 151](#example-151)

    <![CDATA[
              function matchwo(a,b)
              {
                if (a
              < b &&
              a < 0) then {
                 
              return 1;

                } else
              {

                 
              return 0;
                }
              }
              ]]>
              okay
            

    <![CDATA[
              function matchwo(a,b)
              {
                if (a
              < b &&
              a < 0) then {
                 
              return 1;

                } else
              {

                 
              return 0;
                }
              }
              ]]>
              <p>okay</p>
            

The opening tag can be indented 1-3 spaces, but not 4:

[Example 152](#example-152)

      <!-- foo -->

                 
              <!-- foo -->
            

      <!-- foo -->
              <pre><code>&lt;!-- foo --&gt;
              </code></pre>
            

[Example 153](#example-153)

      <div>

                 
              <div>
            

      <div>
              <pre><code>&lt;div&gt;
              </code></pre>
            

An HTML block of types 1–6 can interrupt a paragraph, and need not be
preceded by a blank line.

[Example 154](#example-154)

    Foo
              <div>
              bar
              </div>
            

    <p>Foo</p>
              <div>
              bar
              </div>
            

However, a following blank line is needed, except at the end of a
document, and except for blocks of types 1–5, [above](#html-block):

[Example 155](#example-155)

    <div>
              bar
              </div>
              *foo*
            

    <div>
              bar
              </div>
              *foo*
            

HTML blocks of type 7 cannot interrupt a paragraph:

[Example 156](#example-156)

    Foo
              <a href="bar">
              baz
            

    <p>Foo
              <a href="bar">
              baz</p>
            

This rule differs from John Gruber’s original Markdown syntax
specification, which says:

> The only restrictions are that block-level HTML elements — e.g.
> `<div>`, `<table>`, `<pre>`, `<p>`, etc. — must be separated from
> surrounding content by blank lines, and the start and end tags of the
> block should not be indented with tabs or spaces.

In some ways Gruber’s rule is more restrictive than the one given here:

- It requires that an HTML block be preceded by a blank line.
- It does not allow the start tag to be indented.
- It requires a matching end tag, which it also does not allow to be
  indented.

Most Markdown implementations (including some of Gruber’s own) do not
respect all of these restrictions.

There is one respect, however, in which Gruber’s rule is more liberal
than the one given here, since it allows blank lines to occur inside an
HTML block. There are two reasons for disallowing them here. First, it
removes the need to parse balanced tags, which is expensive and can
require backtracking from the end of the document if no matching end tag
is found. Second, it provides a very simple and flexible way of
including Markdown content inside HTML tags: simply separate the
Markdown from the HTML using blank lines:

Compare:

[Example 157](#example-157)

    <div>

              *Emphasized* text.

              </div>
            

    <div>
              <p><em>Emphasized</em> text.</p>
              </div>
            

[Example 158](#example-158)

    <div>
              *Emphasized* text.
              </div>
            

    <div>
              *Emphasized* text.
              </div>
            

Some Markdown implementations have adopted a convention of interpreting
content inside tags as text if the open tag has the attribute
`markdown=1`. The rule given above seems a simpler and more elegant way
of achieving the same expressive power, which is also much simpler to
parse.

The main potential drawback is that one can no longer paste HTML blocks
into Markdown documents with 100% reliability. However, *in most cases*
this will work fine, because the blank lines in HTML are usually
followed by HTML block tags. For example:

[Example 159](#example-159)

    <table>

              <tr>

              <td>
              Hi
              </td>

              </tr>

              </table>
            

    <table>
              <tr>
              <td>
              Hi
              </td>
              </tr>
              </table>
            

There are problems, however, if the inner tags are indented *and*
separated by spaces, as then they will be interpreted as an indented
code block:

[Example 160](#example-160)

    <table>

                <tr>

                 
              <td>
                 
                Hi
                 
              </td>

                </tr>

              </table>
            

    <table>
                <tr>
              <pre><code>&lt;td&gt;
                Hi
              &lt;/td&gt;
              </code></pre>
                </tr>
              </table>
            

Fortunately, blank lines are usually not necessary and can be deleted.
The exception is inside `<pre>` tags, but as described
[above](#html-blocks), raw HTML blocks starting with `<pre>` *can*
contain blank lines.

## <a href="#TOC" class="toc-link"></a><span class="number">4.7</span>Link reference definitions

A <a href="#link-reference-definition" id="link-reference-definition"
class="definition">link reference definition</a> consists of a [link
label](#link-label), indented up to three spaces, followed by a colon
(`:`), optional [whitespace](#whitespace) (including up to one [line
ending](#line-ending)), a [link destination](#link-destination),
optional [whitespace](#whitespace) (including up to one [line
ending](#line-ending)), and an optional [link title](#link-title), which
if it is present must be separated from the [link
destination](#link-destination) by [whitespace](#whitespace). No further
[non-whitespace characters](#non-whitespace-character) may occur on the
line.

A [link reference definition](#link-reference-definition) does not
correspond to a structural element of a document. Instead, it defines a
label which can be used in [reference links](#reference-link) and
reference-style [images](#images) elsewhere in the document. [Link
reference definitions](#link-reference-definitions) can come either
before or after the links that use them.

[Example 161](#example-161)

    [foo]: /url
              "title"

              [foo]
            

    <p><a href="/url" title="title">foo</a></p>
            

[Example 162](#example-162)

      
              [foo]: 
                 
                /url  
                 
                    
              'the title'  

              [foo]
            

    <p><a href="/url" title="the title">foo</a></p>
            

[Example 163](#example-163)

    [Foo*bar\]]:my_(url) 'title
              (with parens)'

              [Foo*bar\]]
            

    <p><a href="my_(url)" title="title (with
              parens)">Foo*bar]</a></p>
            

[Example 164](#example-164)

    [Foo bar]:
              <my url>
              'title'

              [Foo bar]
            

    <p><a href="my%20url" title="title">Foo bar</a></p>
            

The title may extend over multiple lines:

[Example 165](#example-165)

    [foo]: /url '
              title
              line1
              line2
              '

              [foo]
            

    <p><a href="/url" title="
              title
              line1
              line2
              ">foo</a></p>
            

However, it may not contain a [blank line](#blank-line):

[Example 166](#example-166)

    [foo]: /url 'title

              with blank line'

              [foo]
            

    <p>[foo]: /url
              'title</p>
              <p>with blank line'</p>
              <p>[foo]</p>
            

The title may be omitted:

[Example 167](#example-167)

    [foo]:
              /url

              [foo]
            

    <p><a
              href="/url">foo</a></p>
            

The link destination may not be omitted:

[Example 168](#example-168)

    [foo]:

              [foo]
            

    <p>[foo]:</p>
              <p>[foo]</p>
            

However, an empty link destination may be specified using angle
brackets:

[Example 169](#example-169)

    [foo]: <>

              [foo]
            

    <p><a
              href="">foo</a></p>
            

The title must be separated from the link destination by whitespace:

[Example 170](#example-170)

    [foo]: <bar>(baz)

              [foo]
            

    <p>[foo]: <bar>(baz)</p>
              <p>[foo]</p>
            

Both title and destination can contain backslash escapes and literal
backslashes:

[Example 171](#example-171)

    [foo]: /url\bar\*baz
              "foo\"bar\baz"

              [foo]
            

    <p><a href="/url%5Cbar*baz" title="foo&quot;bar\baz">foo</a></p>
            

A link can come before its corresponding definition:

[Example 172](#example-172)

    [foo]

              [foo]: url
            

    <p><a
              href="url">foo</a></p>
            

If there are several matching definitions, the first one takes
precedence:

[Example 173](#example-173)

    [foo]

              [foo]: first
              [foo]: second
            

    <p><a
              href="first">foo</a></p>
            

As noted in the section on [Links](#links), matching of labels is
case-insensitive (see [matches](#matches)).

[Example 174](#example-174)

    [FOO]: /url

              [Foo]
            

    <p><a
              href="/url">Foo</a></p>
            

[Example 175](#example-175)

    [ΑΓΩ]: /φου

              [αγω]
            

    <p><a
              href="/%CF%86%CE%BF%CF%85">αγω</a></p>
            

Here is a link reference definition with no corresponding link. It
contributes nothing to the document.

[Example 176](#example-176)

    [foo]: /url
            

Here is another one:

[Example 177](#example-177)

    [
              foo
              ]: /url
              bar
            

    <p>bar</p>
            

This is not a link reference definition, because there are
[non-whitespace characters](#non-whitespace-character) after the title:

[Example 178](#example-178)

    [foo]: /url
              "title" ok
            

    <p>[foo]: /url
              &quot;title&quot; ok</p>
            

This is a link reference definition, but it has no title:

[Example 179](#example-179)

    [foo]: /url
              "title" ok
            

    <p>&quot;title&quot; ok</p>
            

This is not a link reference definition, because it is indented four
spaces:

[Example 180](#example-180)

      
               [foo]: /url
              "title"

              [foo]
            

    <pre><code>[foo]: /url
              &quot;title&quot;
              </code></pre>
              <p>[foo]</p>
            

This is not a link reference definition, because it occurs inside a code
block:

[Example 181](#example-181)

    ```
              [foo]: /url
              ```

              [foo]
            

    <pre><code>[foo]: /url
              </code></pre>
              <p>[foo]</p>
            

A [link reference definition](#link-reference-definition) cannot
interrupt a paragraph.

[Example 182](#example-182)

    Foo
              [bar]: /baz

              [bar]
            

    <p>Foo
              [bar]: /baz</p>
              <p>[bar]</p>
            

However, it can directly follow other block elements, such as headings
and thematic breaks, and it need not be followed by a blank line.

[Example 183](#example-183)

    # [Foo]
              [foo]: /url
              > bar
            

    <h1><a
              href="/url">Foo</a></h1>
              <blockquote>
              <p>bar</p>
              </blockquote>
            

[Example 184](#example-184)

    [foo]: /url
              bar
              ===
              [foo]
            

    <h1>bar</h1>
              <p><a href="/url">foo</a></p>
            

[Example 185](#example-185)

    [foo]: /url
              ===
              [foo]
            

    <p>===
              <a href="/url">foo</a></p>
            

Several [link reference definitions](#link-reference-definitions) can
occur one after another, without intervening blank lines.

[Example 186](#example-186)

    [foo]: /foo-url
              "foo"
              [bar]: /bar-url
                "bar"
              [baz]: /baz-url

              [foo],
              [bar],
              [baz]
            

    <p><a href="/foo-url" title="foo">foo</a>,
              <a href="/bar-url"
              title="bar">bar</a>,
              <a href="/baz-url">baz</a></p>
            

[Link reference definitions](#link-reference-definitions) can occur
inside block containers, like lists and block quotations. They affect
the entire document, not just the container in which they are defined:

[Example 187](#example-187)

    [foo]

              > [foo]: /url
            

    <p><a
              href="/url">foo</a></p>
              <blockquote>
              </blockquote>
            

Whether something is a [link reference
definition](#link-reference-definition) is independent of whether the
link reference it defines is used in the document. Thus, for example,
the following document contains just a link reference definition, and no
visible content:

[Example 188](#example-188)

    [foo]: /url
            

## <a href="#TOC" class="toc-link"></a><span class="number">4.8</span>Paragraphs

A sequence of non-blank lines that cannot be interpreted as other kinds
of blocks forms a
<a href="#paragraph" id="paragraph" class="definition">paragraph</a>.
The contents of the paragraph are the result of parsing the paragraph’s
raw content as inlines. The paragraph’s raw content is formed by
concatenating the lines and removing initial and final
[whitespace](#whitespace).

A simple example with two paragraphs:

[Example 189](#example-189)

    aaa

              bbb
            

    <p>aaa</p>
              <p>bbb</p>
            

Paragraphs can contain multiple lines, but no blank lines:

[Example 190](#example-190)

    aaa
              bbb

              ccc
              ddd
            

    <p>aaa
              bbb</p>
              <p>ccc
              ddd</p>
            

Multiple blank lines between paragraph have no effect:

[Example 191](#example-191)

    aaa


              bbb
            

    <p>aaa</p>
              <p>bbb</p>
            

Leading spaces are skipped:

[Example 192](#example-192)

      aaa
               bbb
            

    <p>aaa
              bbb</p>
            

Lines after the first may be indented any amount, since indented code
blocks cannot interrupt paragraphs.

[Example 193](#example-193)

    aaa
                 
                    
                bbb
                 
                    
                    
                    
                    
                    
              ccc
            

    <p>aaa
              bbb
              ccc</p>
            

However, the first line may be indented at most three spaces, or an
indented code block will be triggered:

[Example 194](#example-194)

      
              aaa
              bbb
            

    <p>aaa
              bbb</p>
            

[Example 195](#example-195)

      
               aaa
              bbb
            

    <pre><code>aaa
              </code></pre>
              <p>bbb</p>
            

Final spaces are stripped before inline parsing, so a paragraph that
ends with two or more spaces will not end with a [hard line
break](#hard-line-break):

[Example 196](#example-196)

    aaa     
              bbb   
               
            

    <p>aaa<br />
              bbb</p>
            

## <a href="#TOC" class="toc-link"></a><span class="number">4.9</span>Blank lines

[Blank lines](#blank-line) between block-level elements are ignored,
except for the role they play in determining whether a [list](#list) is
[tight](#tight) or [loose](#loose).

Blank lines at the beginning and end of the document are also ignored.

[Example 197](#example-197)

      

              aaa
                

              # aaa

                
            

    <p>aaa</p>
              <h1>aaa</h1>
            

## <a href="#TOC" class="toc-link"></a><span class="number">4.10</span>Tables (extension)

GFM enables the `table` extension, where an additional leaf block type
is available.

A <a href="#table" id="table" class="definition">table</a> is an
arrangement of data with rows and columns, consisting of a single header
row, a [delimiter row](#delimiter-row) separating the header from the
data, and zero or more data rows.

Each row consists of cells containing arbitrary text, in which
[inlines](#inlines) are parsed, separated by pipes (`|`). A leading and
trailing pipe is also recommended for clarity of reading, and if there’s
otherwise parsing ambiguity. Spaces between pipes and cell content are
trimmed. Block-level elements cannot be inserted in a table.

The
<a href="#delimiter-row" id="delimiter-row" class="definition">delimiter
row</a> consists of cells whose only content are hyphens (`-`), and
optionally, a leading or trailing colon (`:`), or both, to indicate
left, right, or center alignment respectively.

[Example 198](#example-198)

    | foo | bar |
                | --- | --- |
                | baz | bim |
              

    <table>
                <thead>
                <tr>
                <th>foo</th>
                <th>bar</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                <td>baz</td>
                <td>bim</td>
                </tr>
                </tbody>
                </table>
              

Cells in one column don’t need to match length, though it’s easier to
read if they are. Likewise, use of leading and trailing pipes may be
inconsistent:

[Example 199](#example-199)

    | abc | defghi |
                :-: | -----------:
                bar | baz
              

    <table>
                <thead>
                <tr>
                <th align="center">abc</th>
                <th align="right">defghi</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                <td align="center">bar</td>
                <td align="right">baz</td>
                </tr>
                </tbody>
                </table>
              

Include a pipe in a cell’s content by escaping it, including inside
other inline spans:

[Example 200](#example-200)

    | f\|oo  |
                | ------ |
                | b `\|` az |
                | b **\|** im |
              

    <table>
                <thead>
                <tr>
                <th>f|oo</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                <td>b <code>|</code> az</td>
                </tr>
                <tr>
                <td>b <strong>|</strong>
                im</td>
                </tr>
                </tbody>
                </table>
              

The table is broken at the first empty line, or beginning of another
block-level structure:

[Example 201](#example-201)

    | abc | def |
                | --- | --- |
                | bar | baz |
                > bar
              

    <table>
                <thead>
                <tr>
                <th>abc</th>
                <th>def</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                <td>bar</td>
                <td>baz</td>
                </tr>
                </tbody>
                </table>
                <blockquote>
                <p>bar</p>
                </blockquote>
              

[Example 202](#example-202)

    | abc | def |
                | --- | --- |
                | bar | baz |
                bar

                bar
              

    <table>
                <thead>
                <tr>
                <th>abc</th>
                <th>def</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                <td>bar</td>
                <td>baz</td>
                </tr>
                <tr>
                <td>bar</td>
                <td></td>
                </tr>
                </tbody>
                </table>
                <p>bar</p>
              

The header row must match the [delimiter row](#delimiter-row) in the
number of cells. If not, a table will not be recognized:

[Example 203](#example-203)

    | abc | def |
                | --- |
                | bar |
              

    <p>| abc | def |
                | --- |
                | bar |</p>
              

The remainder of the table’s rows may vary in the number of cells. If
there are a number of cells fewer than the number of cells in the header
row, empty cells are inserted. If there are greater, the excess is
ignored:

[Example 204](#example-204)

    | abc | def |
                | --- | --- |
                | bar |
                | bar | baz | boo |
              

    <table>
                <thead>
                <tr>
                <th>abc</th>
                <th>def</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                <td>bar</td>
                <td></td>
                </tr>
                <tr>
                <td>bar</td>
                <td>baz</td>
                </tr>
                </tbody>
                </table>
              

If there are no rows in the body, no `<tbody>` is generated in HTML
output:

[Example 205](#example-205)

    | abc | def |
                | --- | --- |
              

    <table>
                <thead>
                <tr>
                <th>abc</th>
                <th>def</th>
                </tr>
                </thead>
                </table>
              

# <span class="number">5</span>Container blocks

A [container block](#container-blocks) is a block that has other blocks
as its contents. There are two basic kinds of container blocks: [block
quotes](#block-quotes) and [list items](#list-items). [Lists](#list) are
meta-containers for [list items](#list-items).

We define the syntax for container blocks recursively. The general form
of the definition is:

> If X is a sequence of blocks, then the result of transforming X in
> such-and-such a way is a container of type Y with these blocks as its
> content.

So, we explain what counts as a block quote or list item by explaining
how these can be *generated* from their contents. This should suffice to
define the syntax, although it does not give a recipe for *parsing*
these constructions. (A recipe is provided below in the section entitled
[A parsing strategy](#appendix-a-parsing-strategy).)

## <a href="#TOC" class="toc-link"></a><span class="number">5.1</span>Block quotes

A <a href="#block-quote-marker" id="block-quote-marker"
class="definition">block quote marker</a> consists of 0-3 spaces of
initial indent, plus (a) the character `>` together with a following
space, or (b) a single character `>` not followed by a space.

The following rules define [block quotes](#block-quotes):

1.  **Basic case.** If a string of lines *Ls* constitute a sequence of
    blocks *Bs*, then the result of prepending a [block quote
    marker](#block-quote-marker) to the beginning of each line in *Ls*
    is a [block quote](#block-quotes) containing *Bs*.

2.  **Laziness.** If a string of lines *Ls* constitute a [block
    quote](#block-quotes) with contents *Bs*, then the result of
    deleting the initial [block quote marker](#block-quote-marker) from
    one or more lines in which the next [non-whitespace
    character](#non-whitespace-character) after the [block quote
    marker](#block-quote-marker) is [paragraph continuation
    text](#paragraph-continuation-text) is a block quote with *Bs* as
    its content.
    <a href="#paragraph-continuation-text" id="paragraph-continuation-text"
    class="definition">Paragraph continuation text</a> is text that will
    be parsed as part of the content of a paragraph, but does not occur
    at the beginning of the paragraph.

3.  **Consecutiveness.** A document cannot contain two [block
    quotes](#block-quotes) in a row unless there is a [blank
    line](#blank-line) between them.

Nothing else counts as a [block quote](#block-quotes).

Here is a simple example:

[Example 206](#example-206)

    > # Foo
              > bar
              > baz
            

    <blockquote>
              <h1>Foo</h1>
              <p>bar
              baz</p>
              </blockquote>
            

The spaces after the `>` characters can be omitted:

[Example 207](#example-207)

    ># Foo
              >bar
              > baz
            

    <blockquote>
              <h1>Foo</h1>
              <p>bar
              baz</p>
              </blockquote>
            

The `>` characters can be indented 1-3 spaces:

[Example 208](#example-208)

      
              > # Foo
                 >
              bar
               > baz
            

    <blockquote>
              <h1>Foo</h1>
              <p>bar
              baz</p>
              </blockquote>
            

Four spaces gives us a code block:

[Example 209](#example-209)

      
               > # Foo
                 
              > bar
                 
              > baz
            

    <pre><code>&gt; #
              Foo
              &gt; bar
              &gt; baz
              </code></pre>
            

The Laziness clause allows us to omit the `>` before [paragraph
continuation text](#paragraph-continuation-text):

[Example 210](#example-210)

    > # Foo
              > bar
              baz
            

    <blockquote>
              <h1>Foo</h1>
              <p>bar
              baz</p>
              </blockquote>
            

A block quote can contain some lazy and some non-lazy continuation
lines:

[Example 211](#example-211)

    > bar
              baz
              > foo
            

    <blockquote>
              <p>bar
              baz
              foo</p>
              </blockquote>
            

Laziness only applies to lines that would have been continuations of
paragraphs had they been prepended with [block quote
markers](#block-quote-marker). For example, the `> ` cannot be omitted
in the second line of

    > foo
          > ---
        

without changing the meaning:

[Example 212](#example-212)

    > foo
              ---
            

    <blockquote>
              <p>foo</p>
              </blockquote>
              <hr />
            

Similarly, if we omit the `> ` in the second line of

    > - foo
          > - bar
        

then the block quote ends after the first line:

[Example 213](#example-213)

    > - foo
              - bar
            

    <blockquote>
              <ul>
              <li>foo</li>
              </ul>
              </blockquote>
              <ul>
              <li>bar</li>
              </ul>
            

For the same reason, we can’t omit the `> ` in front of subsequent lines
of an indented or fenced code block:

[Example 214](#example-214)

    >     foo
                 
              bar
            

    <blockquote>
              <pre><code>foo
              </code></pre>
              </blockquote>
              <pre><code>bar
              </code></pre>
            

[Example 215](#example-215)

    > ```
              foo
              ```
            

    <blockquote>
              <pre><code></code></pre>
              </blockquote>
              <p>foo</p>
              <pre><code></code></pre>
            

Note that in the following case, we have a [lazy continuation
line](#lazy-continuation-line):

[Example 216](#example-216)

    > foo
                 
              - bar
            

    <blockquote>
              <p>foo
              - bar</p>
              </blockquote>
            

To see why, note that in

    > foo
          > - bar
        

the `- bar` is indented too far to start a list, and can’t be an
indented code block because indented code blocks cannot interrupt
paragraphs, so it is [paragraph continuation
text](#paragraph-continuation-text).

A block quote can be empty:

[Example 217](#example-217)

    >
            

    <blockquote>
              </blockquote>
            

[Example 218](#example-218)

    >
              >  
              > 
            

    <blockquote>
              </blockquote>
            

A block quote can have initial or final blank lines:

[Example 219](#example-219)

    >
              > foo
              >  
            

    <blockquote>
              <p>foo</p>
              </blockquote>
            

A blank line always separates block quotes:

[Example 220](#example-220)

    > foo

              > bar
            

    <blockquote>
              <p>foo</p>
              </blockquote>
              <blockquote>
              <p>bar</p>
              </blockquote>
            

(Most current Markdown implementations, including John Gruber’s original
`Markdown.pl`, will parse this example as a single block quote with two
paragraphs. But it seems better to allow the author to decide whether
two block quotes or one are wanted.)

Consecutiveness means that if we put these block quotes together, we get
a single block quote:

[Example 221](#example-221)

    > foo
              > bar
            

    <blockquote>
              <p>foo
              bar</p>
              </blockquote>
            

To get a block quote with two paragraphs, use:

[Example 222](#example-222)

    > foo
              >
              > bar
            

    <blockquote>
              <p>foo</p>
              <p>bar</p>
              </blockquote>
            

Block quotes can interrupt paragraphs:

[Example 223](#example-223)

    foo
              > bar
            

    <p>foo</p>
              <blockquote>
              <p>bar</p>
              </blockquote>
            

In general, blank lines are not needed before or after block quotes:

[Example 224](#example-224)

    > aaa
              ***
              > bbb
            

    <blockquote>
              <p>aaa</p>
              </blockquote>
              <hr />
              <blockquote>
              <p>bbb</p>
              </blockquote>
            

However, because of laziness, a blank line is needed between a block
quote and a following paragraph:

[Example 225](#example-225)

    > bar
              baz
            

    <blockquote>
              <p>bar
              baz</p>
              </blockquote>
            

[Example 226](#example-226)

    > bar

              baz
            

    <blockquote>
              <p>bar</p>
              </blockquote>
              <p>baz</p>
            

[Example 227](#example-227)

    > bar
              >
              baz
            

    <blockquote>
              <p>bar</p>
              </blockquote>
              <p>baz</p>
            

It is a consequence of the Laziness rule that any number of initial `>`s
may be omitted on a continuation line of a nested block quote:

[Example 228](#example-228)

    > > > foo
              bar
            

    <blockquote>
              <blockquote>
              <blockquote>
              <p>foo
              bar</p>
              </blockquote>
              </blockquote>
              </blockquote>
            

[Example 229](#example-229)

    >>> foo
              > bar
              >>baz
            

    <blockquote>
              <blockquote>
              <blockquote>
              <p>foo
              bar
              baz</p>
              </blockquote>
              </blockquote>
              </blockquote>
            

When including an indented code block in a block quote, remember that
the [block quote marker](#block-quote-marker) includes both the `>` and
a following space. So *five spaces* are needed after the `>`:

[Example 230](#example-230)

    >     code

              >   
              not code
            

    <blockquote>
              <pre><code>code
              </code></pre>
              </blockquote>
              <blockquote>
              <p>not code</p>
              </blockquote>
            

## <a href="#TOC" class="toc-link"></a><span class="number">5.2</span>List items

A <a href="#list-marker" id="list-marker" class="definition">list
marker</a> is a [bullet list marker](#bullet-list-marker) or an [ordered
list marker](#ordered-list-marker).

A <a href="#bullet-list-marker" id="bullet-list-marker"
class="definition">bullet list marker</a> is a `-`, `+`, or `*`
character.

An <a href="#ordered-list-marker" id="ordered-list-marker"
class="definition">ordered list marker</a> is a sequence of 1–9 arabic
digits (`0-9`), followed by either a `.` character or a `)` character.
(The reason for the length limit is that with 10 digits we start seeing
integer overflows in some browsers.)

The following rules define [list items](#list-items):

1.  **Basic case.** If a sequence of lines *Ls* constitute a sequence of
    blocks *Bs* starting with a [non-whitespace
    character](#non-whitespace-character), and *M* is a list marker of
    width *W* followed by 1 ≤ *N* ≤ 4 spaces, then the result of
    prepending *M* and the following spaces to the first line of *Ls*,
    and indenting subsequent lines of *Ls* by *W + N* spaces, is a list
    item with *Bs* as its contents. The type of the list item (bullet or
    ordered) is determined by the type of its list marker. If the list
    item is ordered, then it is also assigned a start number, based on
    the ordered list marker.

    Exceptions:

    1.  When the first list item in a [list](#list) interrupts a
        paragraph—that is, when it starts on a line that would otherwise
        count as [paragraph continuation
        text](#paragraph-continuation-text)—then (a) the lines *Ls* must
        not begin with a blank line, and (b) if the list item is
        ordered, the start number must be 1.
    2.  If any line is a [thematic break](#thematic-break) then that
        line is not a list item.

For example, let *Ls* be the lines

[Example 231](#example-231)

    A paragraph
              with two lines.

                 
              indented code

              > A block quote.
            

    <p>A paragraph
              with two lines.</p>
              <pre><code>indented code
              </code></pre>
              <blockquote>
              <p>A block quote.</p>
              </blockquote>
            

And let *M* be the marker `1.`, and *N* = 2. Then rule \#1 says that the
following is an ordered list item with start number 1, and the same
contents as *Ls*:

[Example 232](#example-232)

    1.  A paragraph
                 
              with two lines.

                 
                  indented code

                 
              > A block quote.
            

    <ol>
              <li>
              <p>A paragraph
              with two lines.</p>
              <pre><code>indented code
              </code></pre>
              <blockquote>
              <p>A block quote.</p>
              </blockquote>
              </li>
              </ol>
            

The most important thing to notice is that the position of the text
after the list marker determines how much indentation is needed in
subsequent blocks in the list item. If the list marker takes up two
spaces, and there are three spaces between the list marker and the next
[non-whitespace character](#non-whitespace-character), then blocks must
be indented five spaces in order to fall under the list item.

Here are some examples showing how far content must be indented to be
put under the list item:

[Example 233](#example-233)

    - one

               two
            

    <ul>
              <li>one</li>
              </ul>
              <p>two</p>
            

[Example 234](#example-234)

    - one

                two
            

    <ul>
              <li>
              <p>one</p>
              <p>two</p>
              </li>
              </ul>
            

[Example 235](#example-235)

     - 
                one

                 
               two
            

    <ul>
              <li>one</li>
              </ul>
              <pre><code> two
              </code></pre>
            

[Example 236](#example-236)

     - 
                one

                 
                two
            

    <ul>
              <li>
              <p>one</p>
              <p>two</p>
              </li>
              </ul>
            

It is tempting to think of this in terms of columns: the continuation
blocks must be indented at least to the column of the first
[non-whitespace character](#non-whitespace-character) after the list
marker. However, that is not quite right. The spaces after the list
marker determine how much relative indentation is needed. Which column
this indentation reaches will depend on how the list item is embedded in
other constructions, as shown by this example:

[Example 237](#example-237)

      
              > > 1.  one
              >>
              >>     two
            

    <blockquote>
              <blockquote>
              <ol>
              <li>
              <p>one</p>
              <p>two</p>
              </li>
              </ol>
              </blockquote>
              </blockquote>
            

Here `two` occurs in the same column as the list marker `1.`, but is
actually contained in the list item, because there is sufficient
indentation after the last containing blockquote marker.

The converse is also possible. In the following example, the word `two`
occurs far to the right of the initial text of the list item, `one`, but
it is not considered part of the list item, because it is not indented
far enough past the blockquote marker:

[Example 238](#example-238)

    >>- one
              >>
                > 
              > two
            

    <blockquote>
              <blockquote>
              <ul>
              <li>one</li>
              </ul>
              <p>two</p>
              </blockquote>
              </blockquote>
            

Note that at least one space is needed between the list marker and any
following content, so these are not list items:

[Example 239](#example-239)

    -one

              2.two
            

    <p>-one</p>
              <p>2.two</p>
            

A list item may contain blocks that are separated by more than one blank
line.

[Example 240](#example-240)

    - foo


                bar
            

    <ul>
              <li>
              <p>foo</p>
              <p>bar</p>
              </li>
              </ul>
            

A list item may contain any kind of block:

[Example 241](#example-241)

    1.  foo

                 
              ```
                 
              bar
                 
              ```

                 
              baz

                 
              > bam
            

    <ol>
              <li>
              <p>foo</p>
              <pre><code>bar
              </code></pre>
              <p>baz</p>
              <blockquote>
              <p>bam</p>
              </blockquote>
              </li>
              </ol>
            

A list item that contains an indented code block will preserve empty
lines within the code block verbatim.

[Example 242](#example-242)

    - Foo

                 
                bar


                 
                baz
            

    <ul>
              <li>
              <p>Foo</p>
              <pre><code>bar


              baz
              </code></pre>
              </li>
              </ul>
            

Note that ordered list start numbers must be nine digits or less:

[Example 243](#example-243)

    123456789. ok
            

    <ol start="123456789">
              <li>ok</li>
              </ol>
            

[Example 244](#example-244)

    1234567890. not ok
            

    <p>1234567890. not
              ok</p>
            

A start number may begin with 0s:

[Example 245](#example-245)

    0. ok
            

    <ol start="0">
              <li>ok</li>
              </ol>
            

[Example 246](#example-246)

    003. ok
            

    <ol start="3">
              <li>ok</li>
              </ol>
            

A start number may not be negative:

[Example 247](#example-247)

    -1. not ok
            

    <p>-1. not
              ok</p>
            

1.  **Item starting with indented code.** If a sequence of lines *Ls*
    constitute a sequence of blocks *Bs* starting with an indented code
    block, and *M* is a list marker of width *W* followed by one space,
    then the result of prepending *M* and the following space to the
    first line of *Ls*, and indenting subsequent lines of *Ls* by *W +
    1* spaces, is a list item with *Bs* as its contents. If a line is
    empty, then it need not be indented. The type of the list item
    (bullet or ordered) is determined by the type of its list marker. If
    the list item is ordered, then it is also assigned a start number,
    based on the ordered list marker.

An indented code block will have to be indented four spaces beyond the
edge of the region where text will be included in the list item. In the
following case that is 6 spaces:

[Example 248](#example-248)

    - foo

                 
                bar
            

    <ul>
              <li>
              <p>foo</p>
              <pre><code>bar
              </code></pre>
              </li>
              </ul>
            

And in this case it is 11 spaces:

[Example 249](#example-249)

      10.  foo

                 
                    
              bar
            

    <ol start="10">
              <li>
              <p>foo</p>
              <pre><code>bar
              </code></pre>
              </li>
              </ol>
            

If the *first* block in the list item is an indented code block, then by
rule \#2, the contents must be indented *one* space after the list
marker:

[Example 250](#example-250)

      
               indented code

              paragraph

                 
              more code
            

    <pre><code>indented code
              </code></pre>
              <p>paragraph</p>
              <pre><code>more code
              </code></pre>
            

[Example 251](#example-251)

    1.     indented
              code

                 paragraph

                 
                 more code
            

    <ol>
              <li>
              <pre><code>indented code
              </code></pre>
              <p>paragraph</p>
              <pre><code>more code
              </code></pre>
              </li>
              </ol>
            

Note that an additional space indent is interpreted as space inside the
code block:

[Example 252](#example-252)

    1.     
              indented code

                 paragraph

                 
                 more code
            

    <ol>
              <li>
              <pre><code> indented code
              </code></pre>
              <p>paragraph</p>
              <pre><code>more code
              </code></pre>
              </li>
              </ol>
            

Note that rules \#1 and \#2 only apply to two cases: (a) cases in which
the lines to be included in a list item begin with a [non-whitespace
character](#non-whitespace-character), and (b) cases in which they begin
with an indented code block. In a case like the following, where the
first block begins with a three-space indent, the rules do not allow us
to form a list item by indenting the whole thing and prepending a list
marker:

[Example 253](#example-253)

      
              foo

              bar
            

    <p>foo</p>
              <p>bar</p>
            

[Example 254](#example-254)

    -  
               foo

                bar
            

    <ul>
              <li>foo</li>
              </ul>
              <p>bar</p>
            

This is not a significant restriction, because when a block begins with
1-3 spaces indent, the indentation can always be removed without a
change in interpretation, allowing rule \#1 to be applied. So, in the
above case:

[Example 255](#example-255)

    -  foo

                 bar
            

    <ul>
              <li>
              <p>foo</p>
              <p>bar</p>
              </li>
              </ul>
            

1.  **Item starting with a blank line.** If a sequence of lines *Ls*
    starting with a single [blank line](#blank-line) constitute a
    (possibly empty) sequence of blocks *Bs*, not separated from each
    other by more than one blank line, and *M* is a list marker of width
    *W*, then the result of prepending *M* to the first line of *Ls*,
    and indenting subsequent lines of *Ls* by *W + 1* spaces, is a list
    item with *Bs* as its contents. If a line is empty, then it need not
    be indented. The type of the list item (bullet or ordered) is
    determined by the type of its list marker. If the list item is
    ordered, then it is also assigned a start number, based on the
    ordered list marker.

Here are some list items that start with a blank line but are not empty:

[Example 256](#example-256)

    -
                foo
              -
                ```
                bar
                ```
              -
                 
                baz
            

    <ul>
              <li>foo</li>
              <li>
              <pre><code>bar
              </code></pre>
              </li>
              <li>
              <pre><code>baz
              </code></pre>
              </li>
              </ul>
            

When the list item starts with a blank line, the number of spaces
following the list marker doesn’t change the required indentation:

[Example 257](#example-257)

    -  
              
                foo
            

    <ul>
              <li>foo</li>
              </ul>
            

A list item can begin with at most one blank line. In the following
example, `foo` is not part of the list item:

[Example 258](#example-258)

    -

                foo
            

    <ul>
              <li></li>
              </ul>
              <p>foo</p>
            

Here is an empty bullet list item:

[Example 259](#example-259)

    - foo
              -
              - bar
            

    <ul>
              <li>foo</li>
              <li></li>
              <li>bar</li>
              </ul>
            

It does not matter whether there are spaces following the [list
marker](#list-marker):

[Example 260](#example-260)

    - foo
              -   
              - bar
            

    <ul>
              <li>foo</li>
              <li></li>
              <li>bar</li>
              </ul>
            

Here is an empty ordered list item:

[Example 261](#example-261)

    1. foo
              2.
              3. bar
            

    <ol>
              <li>foo</li>
              <li></li>
              <li>bar</li>
              </ol>
            

A list may start or end with an empty list item:

[Example 262](#example-262)

    *
            

    <ul>
              <li></li>
              </ul>
            

However, an empty list item cannot interrupt a paragraph:

[Example 263](#example-263)

    foo
              *

              foo
              1.
            

    <p>foo
              *</p>
              <p>foo
              1.</p>
            

1.  **Indentation.** If a sequence of lines *Ls* constitutes a list item
    according to rule \#1, \#2, or \#3, then the result of indenting
    each line of *Ls* by 1-3 spaces (the same for each line) also
    constitutes a list item with the same contents and attributes. If a
    line is empty, then it need not be indented.

Indented one space:

[Example 264](#example-264)

     1.  A paragraph
                 
               with two lines.

                 
                   indented code

                 
               > A block quote.
            

    <ol>
              <li>
              <p>A paragraph
              with two lines.</p>
              <pre><code>indented code
              </code></pre>
              <blockquote>
              <p>A block quote.</p>
              </blockquote>
              </li>
              </ol>
            

Indented two spaces:

[Example 265](#example-265)

      1.  A paragraph
                 
                with two lines.

                 
                    indented
              code

                 
                > A block quote.
            

    <ol>
              <li>
              <p>A paragraph
              with two lines.</p>
              <pre><code>indented code
              </code></pre>
              <blockquote>
              <p>A block quote.</p>
              </blockquote>
              </li>
              </ol>
            

Indented three spaces:

[Example 266](#example-266)

      
              1.  A paragraph
                 
                 with two lines.

                 
                    
              indented code

                 
                 > A block quote.
            

    <ol>
              <li>
              <p>A paragraph
              with two lines.</p>
              <pre><code>indented code
              </code></pre>
              <blockquote>
              <p>A block quote.</p>
              </blockquote>
              </li>
              </ol>
            

Four spaces indent gives a code block:

[Example 267](#example-267)

      
               1.  A paragraph
                 
                  with two lines.

                 
                    
               indented code

                 
                  > A block
              quote.
            

    <pre><code>1. 
              A paragraph
                 
              with two lines.

                 
                  indented code

                 
              &gt; A block
              quote.
              </code></pre>
            

1.  **Laziness.** If a string of lines *Ls* constitute a [list
    item](#list-items) with contents *Bs*, then the result of deleting
    some or all of the indentation from one or more lines in which the
    next [non-whitespace character](#non-whitespace-character) after the
    indentation is [paragraph continuation
    text](#paragraph-continuation-text) is a list item with the same
    contents and attributes. The unindented lines are called
    <a href="#lazy-continuation-line" id="lazy-continuation-line"
    class="definition">lazy continuation line</a>s.

Here is an example with [lazy continuation
lines](#lazy-continuation-line):

[Example 268](#example-268)

      1.  A paragraph
              with two lines.

                 
                    indented
              code

                 
                > A block quote.
            

    <ol>
              <li>
              <p>A paragraph
              with two lines.</p>
              <pre><code>indented code
              </code></pre>
              <blockquote>
              <p>A block quote.</p>
              </blockquote>
              </li>
              </ol>
            

Indentation can be partially deleted:

[Example 269](#example-269)

      1.  A paragraph
                 
              with two lines.
            

    <ol>
              <li>A paragraph
              with two lines.</li>
              </ol>
            

These examples show how laziness can work in nested structures:

[Example 270](#example-270)

    > 1. > Blockquote
              continued here.
            

    <blockquote>
              <ol>
              <li>
              <blockquote>
              <p>Blockquote
              continued here.</p>
              </blockquote>
              </li>
              </ol>
              </blockquote>
            

[Example 271](#example-271)

    > 1. > Blockquote
              > continued here.
            

    <blockquote>
              <ol>
              <li>
              <blockquote>
              <p>Blockquote
              continued here.</p>
              </blockquote>
              </li>
              </ol>
              </blockquote>
            

1.  **That’s all.** Nothing that is not counted as a list item by rules
    \#1–5 counts as a [list item](#list-items).

The rules for sublists follow from the general rules
[above](#list-items). A sublist must be indented the same number of
spaces a paragraph would need to be in order to be included in the list
item.

So, in this case we need two spaces indent:

[Example 272](#example-272)

    - foo
                - bar
                 
              - baz
                 
                - boo
            

    <ul>
              <li>foo
              <ul>
              <li>bar
              <ul>
              <li>baz
              <ul>
              <li>boo</li>
              </ul>
              </li>
              </ul>
              </li>
              </ul>
              </li>
              </ul>
            

One is not enough:

[Example 273](#example-273)

    - foo
               - bar
                - baz
                 -
              boo
            

    <ul>
              <li>foo</li>
              <li>bar</li>
              <li>baz</li>
              <li>boo</li>
              </ul>
            

Here we need four, because the list marker is wider:

[Example 274](#example-274)

    10) foo
                 
              - bar
            

    <ol start="10">
              <li>foo
              <ul>
              <li>bar</li>
              </ul>
              </li>
              </ol>
            

Three is not enough:

[Example 275](#example-275)

    10) foo
                 -
              bar
            

    <ol start="10">
              <li>foo</li>
              </ol>
              <ul>
              <li>bar</li>
              </ul>
            

A list may be the first block in a list item:

[Example 276](#example-276)

    - - foo
            

    <ul>
              <li>
              <ul>
              <li>foo</li>
              </ul>
              </li>
              </ul>
            

[Example 277](#example-277)

    1. - 2. foo
            

    <ol>
              <li>
              <ul>
              <li>
              <ol start="2">
              <li>foo</li>
              </ol>
              </li>
              </ul>
              </li>
              </ol>
            

A list item can contain a heading:

[Example 278](#example-278)

    - # Foo
              - Bar
                ---
                baz
            

    <ul>
              <li>
              <h1>Foo</h1>
              </li>
              <li>
              <h2>Bar</h2>
              baz</li>
              </ul>
            

### <span class="number">5.2.1</span>Motivation

John Gruber’s Markdown spec says the following about list items:

1.  “List markers typically start at the left margin, but may be
    indented by up to three spaces. List markers must be followed by one
    or more spaces or a tab.”

2.  “To make lists look nice, you can wrap items with hanging indents….
    But if you don’t want to, you don’t have to.”

3.  “List items may consist of multiple paragraphs. Each subsequent
    paragraph in a list item must be indented by either 4 spaces or one
    tab.”

4.  “It looks nice if you indent every line of the subsequent
    paragraphs, but here again, Markdown will allow you to be lazy.”

5.  “To put a blockquote within a list item, the blockquote’s `>`
    delimiters need to be indented.”

6.  “To put a code block within a list item, the code block needs to be
    indented twice — 8 spaces or two tabs.”

These rules specify that a paragraph under a list item must be indented
four spaces (presumably, from the left margin, rather than the start of
the list marker, but this is not said), and that code under a list item
must be indented eight spaces instead of the usual four. They also say
that a block quote must be indented, but not by how much; however, the
example given has four spaces indentation. Although nothing is said
about other kinds of block-level content, it is certainly reasonable to
infer that *all* block elements under a list item, including other
lists, must be indented four spaces. This principle has been called the
*four-space rule*.

The four-space rule is clear and principled, and if the reference
implementation `Markdown.pl` had followed it, it probably would have
become the standard. However, `Markdown.pl` allowed paragraphs and
sublists to start with only two spaces indentation, at least on the
outer level. Worse, its behavior was inconsistent: a sublist of an
outer-level list needed two spaces indentation, but a sublist of this
sublist needed three spaces. It is not surprising, then, that different
implementations of Markdown have developed very different rules for
determining what comes under a list item. (Pandoc and python-Markdown,
for example, stuck with Gruber’s syntax description and the four-space
rule, while discount, redcarpet, marked, PHP Markdown, and others
followed `Markdown.pl`’s behavior more closely.)

Unfortunately, given the divergences between implementations, there is
no way to give a spec for list items that will be guaranteed not to
break any existing documents. However, the spec given here should
correctly handle lists formatted with either the four-space rule or the
more forgiving `Markdown.pl` behavior, provided they are laid out in a
way that is natural for a human to read.

The strategy here is to let the width and indentation of the list marker
determine the indentation necessary for blocks to fall under the list
item, rather than having a fixed and arbitrary number. The writer can
think of the body of the list item as a unit which gets indented to the
right enough to fit the list marker (and any indentation on the list
marker). (The laziness rule, \#5, then allows continuation lines to be
unindented if needed.)

This rule is superior, we claim, to any rule requiring a fixed level of
indentation from the margin. The four-space rule is clear but unnatural.
It is quite unintuitive that

    - foo

          bar

          - baz
        

should be parsed as two lists with an intervening paragraph,

    <ul>
          <li>foo</li>
          </ul>
          <p>bar</p>
          <ul>
          <li>baz</li>
          </ul>
        

as the four-space rule demands, rather than a single list,

    <ul>
          <li>
          <p>foo</p>
          <p>bar</p>
          <ul>
          <li>baz</li>
          </ul>
          </li>
          </ul>
        

The choice of four spaces is arbitrary. It can be learned, but it is not
likely to be guessed, and it trips up beginners regularly.

Would it help to adopt a two-space rule? The problem is that such a
rule, together with the rule allowing 1–3 spaces indentation of the
initial list marker, allows text that is indented *less than* the
original list marker to be included in the list item. For example,
`Markdown.pl` parses

     - one

          two
        

as a single list item, with `two` a continuation paragraph:

    <ul>
          <li>
          <p>one</p>
          <p>two</p>
          </li>
          </ul>
        

and similarly

    > - one
          >
          > two
        

as

    <blockquote>
          <ul>
          <li>
          <p>one</p>
          <p>two</p>
          </li>
          </ul>
          </blockquote>
        

This is extremely unintuitive.

Rather than requiring a fixed indent from the margin, we could require a
fixed indent (say, two spaces, or even one space) from the list marker
(which may itself be indented). This proposal would remove the last
anomaly discussed. Unlike the spec presented above, it would count the
following as a list item with a subparagraph, even though the paragraph
`bar` is not indented as far as the first paragraph `foo`:

     10. foo

          bar
        

Arguably this text does read like a list item with `bar` as a
subparagraph, which may count in favor of the proposal. However, on this
proposal indented code would have to be indented six spaces after the
list marker. And this would break a lot of existing Markdown, which has
the pattern:

    1. foo

          indented code
        

where the code is indented eight spaces. The spec above, by contrast,
will parse this text as expected, since the code block’s indentation is
measured from the beginning of `foo`.

The one case that needs special treatment is a list item that *starts*
with indented code. How much indentation is required in that case, since
we don’t have a “first paragraph” to measure from? Rule \#2 simply
stipulates that in such cases, we require one space indentation from the
list marker (and then the normal four spaces for the indented code).
This will match the four-space rule in cases where the list marker plus
its initial indentation takes four spaces (a common case), but diverge
in other cases.

## <a href="#TOC" class="toc-link"></a><span class="number">5.3</span>Task list items (extension)

GFM enables the `tasklist` extension, where an additional processing
step is performed on [list items](#list-items).

A <a href="#task-list-item" id="task-list-item" class="definition">task
list item</a> is a [list item](#list-items) where the first block in it
is a paragraph which begins with a [task list item
marker](#task-list-item-marker) and at least one whitespace character
before any other content.

A <a href="#task-list-item-marker" id="task-list-item-marker"
class="definition">task list item marker</a> consists of an optional
number of spaces, a left bracket (`[`), either a whitespace character or
the letter `x` in either lowercase or uppercase, and then a right
bracket (`]`).

When rendered, the [task list item marker](#task-list-item-marker) is
replaced with a semantic checkbox element; in an HTML output, this would
be an `<input type="checkbox">` element.

If the character between the brackets is a whitespace character, the
checkbox is unchecked. Otherwise, the checkbox is checked.

This spec does not define how the checkbox elements are interacted with:
in practice, implementors are free to render the checkboxes as disabled
or inmutable elements, or they may dynamically handle dynamic
interactions (i.e. checking, unchecking) in the final rendered document.

[Example 279](#example-279)

    - [ ] foo
                - [x] bar
              

    <ul>
                <li><input disabled=""
                type="checkbox"> foo</li>
                <li><input checked=""
                disabled="" type="checkbox">
                bar</li>
                </ul>
              

Task lists can be arbitrarily nested:

[Example 280](#example-280)

    - [x] foo
                  - [
                ] bar
                  - [x]
                baz
                - [ ] bim
              

    <ul>
                <li><input checked=""
                disabled="" type="checkbox">
                foo
                <ul>
                <li><input disabled=""
                type="checkbox"> bar</li>
                <li><input checked=""
                disabled="" type="checkbox">
                baz</li>
                </ul>
                </li>
                <li><input disabled=""
                type="checkbox"> bim</li>
                </ul>
              

## <a href="#TOC" class="toc-link"></a><span class="number">5.4</span>Lists

A <a href="#list" id="list" class="definition">list</a> is a sequence of
one or more list items [of the same type](#of-the-same-type). The list
items may be separated by any number of blank lines.

Two list items are
<a href="#of-the-same-type" id="of-the-same-type" class="definition">of
the same type</a> if they begin with a [list marker](#list-marker) of
the same type. Two list markers are of the same type if (a) they are
bullet list markers using the same character (`-`, `+`, or `*`) or (b)
they are ordered list numbers with the same delimiter (either `.` or
`)`).

A list is an
<a href="#ordered-list" id="ordered-list" class="definition">ordered
list</a> if its constituent list items begin with [ordered list
markers](#ordered-list-marker), and a
<a href="#bullet-list" id="bullet-list" class="definition">bullet
list</a> if its constituent list items begin with [bullet list
markers](#bullet-list-marker).

The <a href="#start-number" id="start-number" class="definition">start
number</a> of an [ordered list](#ordered-list) is determined by the list
number of its initial list item. The numbers of subsequent list items
are disregarded.

A list is <a href="#loose" id="loose" class="definition">loose</a> if
any of its constituent list items are separated by blank lines, or if
any of its constituent list items directly contain two block-level
elements with a blank line between them. Otherwise a list is
<a href="#tight" id="tight" class="definition">tight</a>. (The
difference in HTML output is that paragraphs in a loose list are wrapped
in `<p>` tags, while paragraphs in a tight list are not.)

Changing the bullet or ordered list delimiter starts a new list:

[Example 281](#example-281)

    - foo
              - bar
              + baz
            

    <ul>
              <li>foo</li>
              <li>bar</li>
              </ul>
              <ul>
              <li>baz</li>
              </ul>
            

[Example 282](#example-282)

    1. foo
              2. bar
              3) baz
            

    <ol>
              <li>foo</li>
              <li>bar</li>
              </ol>
              <ol start="3">
              <li>baz</li>
              </ol>
            

In CommonMark, a list can interrupt a paragraph. That is, no blank line
is needed to separate a paragraph from a following list:

[Example 283](#example-283)

    Foo
              - bar
              - baz
            

    <p>Foo</p>
              <ul>
              <li>bar</li>
              <li>baz</li>
              </ul>
            

`Markdown.pl` does not allow this, through fear of triggering a list via
a numeral in a hard-wrapped line:

    The number of windows in my house is
          14. The number of doors is 6.
        

Oddly, though, `Markdown.pl` *does* allow a blockquote to interrupt a
paragraph, even though the same considerations might apply.

In CommonMark, we do allow lists to interrupt paragraphs, for two
reasons. First, it is natural and not uncommon for people to start lists
without blank lines:

    I need to buy
          - new shoes
          - a coat
          - a plane ticket
        

Second, we are attracted to a

> <a href="#principle-of-uniformity" id="principle-of-uniformity"
> class="definition">principle of uniformity</a>: if a chunk of text has
> a certain meaning, it will continue to have the same meaning when put
> into a container block (such as a list item or blockquote).

(Indeed, the spec for [list items](#list-items) and [block
quotes](#block-quotes) presupposes this principle.) This principle
implies that if

     * I need to buy
          - new shoes
          - a coat
          - a plane ticket
        

is a list item containing a paragraph followed by a nested sublist, as
all Markdown implementations agree it is (though the paragraph may be
rendered without `<p>` tags, since the list is “tight”), then

    I need to buy
          - new shoes
          - a coat
          - a plane ticket
        

by itself should be a paragraph followed by a nested sublist.

Since it is well established Markdown practice to allow lists to
interrupt paragraphs inside list items, the [principle of
uniformity](#principle-of-uniformity) requires us to allow this outside
list items as well.
([reStructuredText](http://docutils.sourceforge.net/rst.html) takes a
different approach, requiring blank lines before lists even inside other
list items.)

In order to solve of unwanted lists in paragraphs with hard-wrapped
numerals, we allow only lists starting with `1` to interrupt paragraphs.
Thus,

[Example 284](#example-284)

    The number of windows in my house is
              14.  The number of doors is
              6.
            

    <p>The number of windows in my house is
              14.  The number of doors is
              6.</p>
            

We may still get an unintended result in cases like

[Example 285](#example-285)

    The number of windows in my house is
              1.  The number of doors is
              6.
            

    <p>The number of windows in my house is</p>
              <ol>
              <li>The number of
              doors is 6.</li>
              </ol>
            

but this rule should prevent most spurious list captures.

There can be any number of blank lines between items:

[Example 286](#example-286)

    - foo

              - bar


              - baz
            

    <ul>
              <li>
              <p>foo</p>
              </li>
              <li>
              <p>bar</p>
              </li>
              <li>
              <p>baz</p>
              </li>
              </ul>
            

[Example 287](#example-287)

    - foo
                - bar
                 
              - baz


                 
                bim
            

    <ul>
              <li>foo
              <ul>
              <li>bar
              <ul>
              <li>
              <p>baz</p>
              <p>bim</p>
              </li>
              </ul>
              </li>
              </ul>
              </li>
              </ul>
            

To separate consecutive lists of the same type, or to separate a list
from an indented code block that would otherwise be parsed as a
subparagraph of the final list item, you can insert a blank HTML
comment:

[Example 288](#example-288)

    - foo
              - bar

              <!-- -->

              - baz
              - bim
            

    <ul>
              <li>foo</li>
              <li>bar</li>
              </ul>
              <!-- -->
              <ul>
              <li>baz</li>
              <li>bim</li>
              </ul>
            

[Example 289](#example-289)

    -  
              foo

                 
              notcode

              -   foo

              <!-- -->

                 
              code
            

    <ul>
              <li>
              <p>foo</p>
              <p>notcode</p>
              </li>
              <li>
              <p>foo</p>
              </li>
              </ul>
              <!-- -->
              <pre><code>code
              </code></pre>
            

List items need not be indented to the same level. The following list
items will be treated as items at the same list level, since none is
indented enough to belong to the previous list item:

[Example 290](#example-290)

    - a
               - b
                - c
                 -
              d
                - e
               - f
              - g
            

    <ul>
              <li>a</li>
              <li>b</li>
              <li>c</li>
              <li>d</li>
              <li>e</li>
              <li>f</li>
              <li>g</li>
              </ul>
            

[Example 291](#example-291)

    1. a

                2. b

                 3.
              c
            

    <ol>
              <li>
              <p>a</p>
              </li>
              <li>
              <p>b</p>
              </li>
              <li>
              <p>c</p>
              </li>
              </ol>
            

Note, however, that list items may not be indented more than three
spaces. Here `- e` is treated as a paragraph continuation line, because
it is indented more than three spaces:

[Example 292](#example-292)

    - a
               - b
                - c
                 -
              d
                 
              - e
            

    <ul>
              <li>a</li>
              <li>b</li>
              <li>c</li>
              <li>d
              - e</li>
              </ul>
            

And here, `3. c` is treated as in indented code block, because it is
indented four spaces and preceded by a blank line.

[Example 293](#example-293)

    1. a

                2. b

                 
              3. c
            

    <ol>
              <li>
              <p>a</p>
              </li>
              <li>
              <p>b</p>
              </li>
              </ol>
              <pre><code>3. c
              </code></pre>
            

This is a loose list, because there is a blank line between two of the
list items:

[Example 294](#example-294)

    - a
              - b

              - c
            

    <ul>
              <li>
              <p>a</p>
              </li>
              <li>
              <p>b</p>
              </li>
              <li>
              <p>c</p>
              </li>
              </ul>
            

So is this, with a empty second item:

[Example 295](#example-295)

    * a
              *

              * c
            

    <ul>
              <li>
              <p>a</p>
              </li>
              <li></li>
              <li>
              <p>c</p>
              </li>
              </ul>
            

These are loose lists, even though there is no space between the items,
because one of the items directly contains two block-level elements with
a blank line between them:

[Example 296](#example-296)

    - a
              - b

                c
              - d
            

    <ul>
              <li>
              <p>a</p>
              </li>
              <li>
              <p>b</p>
              <p>c</p>
              </li>
              <li>
              <p>d</p>
              </li>
              </ul>
            

[Example 297](#example-297)

    - a
              - b

                [ref]: /url
              - d
            

    <ul>
              <li>
              <p>a</p>
              </li>
              <li>
              <p>b</p>
              </li>
              <li>
              <p>d</p>
              </li>
              </ul>
            

This is a tight list, because the blank lines are in a code block:

[Example 298](#example-298)

    - a
              - ```
                b


                ```
              - c
            

    <ul>
              <li>a</li>
              <li>
              <pre><code>b


              </code></pre>
              </li>
              <li>c</li>
              </ul>
            

This is a tight list, because the blank line is between two paragraphs
of a sublist. So the sublist is loose while the outer list is tight:

[Example 299](#example-299)

    - a
                - b

                 
              c
              - d
            

    <ul>
              <li>a
              <ul>
              <li>
              <p>b</p>
              <p>c</p>
              </li>
              </ul>
              </li>
              <li>d</li>
              </ul>
            

This is a tight list, because the blank line is inside the block quote:

[Example 300](#example-300)

    * a
                > b
                >
              * c
            

    <ul>
              <li>a
              <blockquote>
              <p>b</p>
              </blockquote>
              </li>
              <li>c</li>
              </ul>
            

This list is tight, because the consecutive block elements are not
separated by blank lines:

[Example 301](#example-301)

    - a
                > b
                ```
                c
                ```
              - d
            

    <ul>
              <li>a
              <blockquote>
              <p>b</p>
              </blockquote>
              <pre><code>c
              </code></pre>
              </li>
              <li>d</li>
              </ul>
            

A single-paragraph list is tight:

[Example 302](#example-302)

    - a
            

    <ul>
              <li>a</li>
              </ul>
            

[Example 303](#example-303)

    - a
                - b
            

    <ul>
              <li>a
              <ul>
              <li>b</li>
              </ul>
              </li>
              </ul>
            

This list is loose, because of the blank line between the two block
elements in the list item:

[Example 304](#example-304)

    1. ```
                 foo
                 ```

                 bar
            

    <ol>
              <li>
              <pre><code>foo
              </code></pre>
              <p>bar</p>
              </li>
              </ol>
            

Here the outer list is loose, the inner list tight:

[Example 305](#example-305)

    * foo
                * bar

                baz
            

    <ul>
              <li>
              <p>foo</p>
              <ul>
              <li>bar</li>
              </ul>
              <p>baz</p>
              </li>
              </ul>
            

[Example 306](#example-306)

    - a
                - b
                - c

              - d
                - e
                - f
            

    <ul>
              <li>
              <p>a</p>
              <ul>
              <li>b</li>
              <li>c</li>
              </ul>
              </li>
              <li>
              <p>d</p>
              <ul>
              <li>e</li>
              <li>f</li>
              </ul>
              </li>
              </ul>
            

# <span class="number">6</span>Inlines

Inlines are parsed sequentially from the beginning of the character
stream to the end (left to right, in left-to-right languages). Thus, for
example, in

[Example 307](#example-307)

    `hi`lo`
            

    <p><code>hi</code>lo`</p>
            

`hi` is parsed as code, leaving the backtick at the end as a literal
backtick.

## <a href="#TOC" class="toc-link"></a><span class="number">6.1</span>Backslash escapes

Any ASCII punctuation character may be backslash-escaped:

[Example 308](#example-308)

    \!\"\#\$\%\&\'\(\)\*\+\,\-\.\/\:\;\<\=\>\?\@\[\\\]\^\_\`\{\|\}\~
            

    <p>!&quot;#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~</p>
            

Backslashes before other characters are treated as literal backslashes:

[Example 309](#example-309)

    \→\A\a\ \3\φ\«
            

    <p>\→\A\a\ \3\φ\«</p>
            

Escaped characters are treated as regular characters and do not have
their usual Markdown meanings:

[Example 310](#example-310)

    \*not emphasized*
              \<br/> not a tag
              \[not a link](/foo)
              \`not code`
              1\. not a list
              \* not a list
              \# not a heading
              \[foo]: /url "not a reference"
              \&ouml; not a
              character entity
            

    <p>*not emphasized*
              &lt;br/&gt; not a tag
              [not a link](/foo)
              `not code`
              1. not a list
              * not a list
              # not a heading
              [foo]: /url &quot;not
              a reference&quot;
              &amp;ouml; not a
              character entity</p>
            

If a backslash is itself escaped, the following character is not:

[Example 311](#example-311)

    \\*emphasis*
            

    <p>\<em>emphasis</em></p>
            

A backslash at the end of the line is a [hard line
break](#hard-line-break):

[Example 312](#example-312)

    foo\
              bar
            

    <p>foo<br />
              bar</p>
            

Backslash escapes do not work in code blocks, code spans, autolinks, or
raw HTML:

[Example 313](#example-313)

    `` \[\` ``
            

    <p><code>\[\`</code></p>
            

[Example 314](#example-314)

      
               \[\]
            

    <pre><code>\[\]
              </code></pre>
            

[Example 315](#example-315)

    ~~~
              \[\]
              ~~~
            

    <pre><code>\[\]
              </code></pre>
            

[Example 316](#example-316)

    <http://example.com?find=\*>
            

    <p><a
              href="http://example.com?find=%5C*">http://example.com?find=\*</a></p>
            

[Example 317](#example-317)

    <a href="/bar\/)">
            

    <a href="/bar\/)">
            

But they work in all other contexts, including URLs and link titles,
link references, and [info strings](#info-string) in [fenced code
blocks](#fenced-code-blocks):

[Example 318](#example-318)

    [foo](/bar\* "ti\*tle")
            

    <p><a href="/bar*" title="ti*tle">foo</a></p>
            

[Example 319](#example-319)

    [foo]

              [foo]: /bar\* "ti\*tle"
            

    <p><a href="/bar*" title="ti*tle">foo</a></p>
            

[Example 320](#example-320)

    ``` foo\+bar
              foo
              ```
            

    <pre><code
              class="language-foo+bar">foo
              </code></pre>
            

## <a href="#TOC" class="toc-link"></a><span class="number">6.2</span>Entity and numeric character references

Valid HTML entity references and numeric character references can be
used in place of the corresponding Unicode character, with the following
exceptions:

- Entity and character references are not recognized in code blocks and
  code spans.

- Entity and character references cannot stand in place of special
  characters that define structural elements in CommonMark. For example,
  although `&#42;` can be used in place of a literal `*` character,
  `&#42;` cannot replace `*` in emphasis delimiters, bullet list
  markers, or thematic breaks.

Conforming CommonMark parsers need not store information about whether a
particular character was represented in the source using a Unicode
character or an entity reference.

<a href="#entity-references" id="entity-references"
class="definition">Entity references</a> consist of `&` + any of the
valid HTML5 entity names + `;`. The document
<https://html.spec.whatwg.org/multipage/entities.json> is used as an
authoritative source for the valid entity references and their
corresponding code points.

[Example 321](#example-321)

    &nbsp; &amp;
              &copy; &AElig; &Dcaron;
              &frac34; &HilbertSpace; &DifferentialD;
              &ClockwiseContourIntegral; &ngE;
            

    <p>  &amp; © Æ Ď
              ¾ ℋ ⅆ
              ∲ ≧̸</p>
            

<a href="#decimal-numeric-character-references"
id="decimal-numeric-character-references" class="definition">Decimal
numeric character references</a> consist of `&#` + a string of 1–7
arabic digits + `;`. A numeric character reference is parsed as the
corresponding Unicode character. Invalid Unicode code points will be
replaced by the REPLACEMENT CHARACTER (`U+FFFD`). For security reasons,
the code point `U+0000` will also be replaced by `U+FFFD`.

[Example 322](#example-322)

    &#35; &#1234;
              &#992; &#0;
            

    <p># Ӓ Ϡ �</p>
            

<a href="#hexadecimal-numeric-character-references"
id="hexadecimal-numeric-character-references"
class="definition">Hexadecimal numeric character references</a> consist
of `&#` + either `X` or `x` + a string of 1-6 hexadecimal digits + `;`.
They too are parsed as the corresponding Unicode character (this time
specified with a hexadecimal numeral instead of decimal).

[Example 323](#example-323)

    &#X22; &#XD06;
              &#xcab;
            

    <p>&quot; ആ
              ಫ</p>
            

Here are some nonentities:

[Example 324](#example-324)

    &nbsp &x;
              &#; &#x;
              &#87654321;
              &#abcdef0;
              &ThisIsNotDefined; &hi?;
            

    <p>&amp;nbsp &amp;x;
              &amp;#; &amp;#x;
              &amp;#87654321;
              &amp;#abcdef0;
              &amp;ThisIsNotDefined; &amp;hi?;</p>
            

Although HTML5 does accept some entity references without a trailing
semicolon (such as `&copy`), these are not recognized here, because it
makes the grammar too ambiguous:

[Example 325](#example-325)

    &copy
            

    <p>&amp;copy</p>
            

Strings that are not on the list of HTML5 named entities are not
recognized as entity references either:

[Example 326](#example-326)

    &MadeUpEntity;
            

    <p>&amp;MadeUpEntity;</p>
            

Entity and numeric character references are recognized in any context
besides code spans or code blocks, including URLs, [link
titles](#link-title), and [fenced code block](#fenced-code-block) [info
strings](#info-string):

[Example 327](#example-327)

    <a
              href="&ouml;&ouml;.html">
            

    <a href="&ouml;&ouml;.html">
            

[Example 328](#example-328)

    [foo](/f&ouml;&ouml;
              "f&ouml;&ouml;")
            

    <p><a href="/f%C3%B6%C3%B6" title="föö">foo</a></p>
            

[Example 329](#example-329)

    [foo]

              [foo]: /f&ouml;&ouml;
              "f&ouml;&ouml;"
            

    <p><a href="/f%C3%B6%C3%B6" title="föö">foo</a></p>
            

[Example 330](#example-330)

    ``` f&ouml;&ouml;
              foo
              ```
            

    <pre><code
              class="language-föö">foo
              </code></pre>
            

Entity and numeric character references are treated as literal text in
code spans and code blocks:

[Example 331](#example-331)

    `f&ouml;&ouml;`
            

    <p><code>f&amp;ouml;&amp;ouml;</code></p>
            

[Example 332](#example-332)

      
               f&ouml;f&ouml;
            

    <pre><code>f&amp;ouml;f&amp;ouml;
              </code></pre>
            

Entity and numeric character references cannot be used in place of
symbols indicating structure in CommonMark documents.

[Example 333](#example-333)

    &#42;foo&#42;
              *foo*
            

    <p>*foo*
              <em>foo</em></p>
            

[Example 334](#example-334)

    &#42; foo

              * foo
            

    <p>* foo</p>
              <ul>
              <li>foo</li>
              </ul>
            

[Example 335](#example-335)

    foo&#10;&#10;bar
            

    <p>foo

              bar</p>
            

[Example 336](#example-336)

    &#9;foo
            

    <p>→foo</p>
            

[Example 337](#example-337)

    [a](url &quot;tit&quot;)
            

    <p>[a](url &quot;tit&quot;)</p>
            

## <a href="#TOC" class="toc-link"></a><span class="number">6.3</span>Code spans

A <a href="#backtick-string" id="backtick-string"
class="definition">backtick string</a> is a string of one or more
backtick characters (`` ` ``) that is neither preceded nor followed by a
backtick.

A <a href="#code-span" id="code-span" class="definition">code span</a>
begins with a backtick string and ends with a backtick string of equal
length. The contents of the code span are the characters between the two
backtick strings, normalized in the following ways:

- First, [line endings](#line-ending) are converted to [spaces](#space).
- If the resulting string both begins *and* ends with a [space](#space)
  character, but does not consist entirely of [space](#space)
  characters, a single [space](#space) character is removed from the
  front and back. This allows you to include code that begins or ends
  with backtick characters, which must be separated by whitespace from
  the opening or closing backtick strings.

This is a simple code span:

[Example 338](#example-338)

    `foo`
            

    <p><code>foo</code></p>
            

Here two backticks are used, because the code contains a backtick. This
example also illustrates stripping of a single leading and trailing
space:

[Example 339](#example-339)

    `` foo ` bar ``
            

    <p><code>foo `
              bar</code></p>
            

This example shows the motivation for stripping leading and trailing
spaces:

[Example 340](#example-340)

    ` `` `
            

    <p><code>``</code></p>
            

Note that only *one* space is stripped:

[Example 341](#example-341)

    `  ``  `
            

    <p><code> ``
              </code></p>
            

The stripping only happens if the space is on both sides of the string:

[Example 342](#example-342)

    ` a`
            

    <p><code> a</code></p>
            

Only [spaces](#space), and not [unicode whitespace](#unicode-whitespace)
in general, are stripped in this way:

[Example 343](#example-343)

    ` b `
            

    <p><code> b </code></p>
            

No stripping occurs if the code span contains only spaces:

[Example 344](#example-344)

    ` `
              `  `
            

    <p><code> </code>
              <code>  </code></p>
            

[Line endings](#line-ending) are treated like spaces:

[Example 345](#example-345)

    ``
              foo
              bar  
              baz
              ``
            

    <p><code>foo bar
                baz</code></p>
            

[Example 346](#example-346)

    ``
              foo 
              ``
            

    <p><code>foo </code></p>
            

Interior spaces are not collapsed:

[Example 347](#example-347)

    `foo   bar 
              baz`
            

    <p><code>foo 
               bar 
              baz</code></p>
            

Note that browsers will typically collapse consecutive spaces when
rendering `<code>` elements, so it is recommended that the following CSS
be used:

    code{white-space: pre-wrap;}
        

Note that backslash escapes do not work in code spans. All backslashes
are treated literally:

[Example 348](#example-348)

    `foo\`bar`
            

    <p><code>foo\</code>bar`</p>
            

Backslash escapes are never needed, because one can always choose a
string of *n* backtick characters as delimiters, where the code does not
contain any strings of exactly *n* backtick characters.

[Example 349](#example-349)

    ``foo`bar``
            

    <p><code>foo`bar</code></p>
            

[Example 350](#example-350)

    ` foo `` bar `
            

    <p><code>foo ``
              bar</code></p>
            

Code span backticks have higher precedence than any other inline
constructs except HTML tags and autolinks. Thus, for example, this is
not parsed as emphasized text, since the second `*` is part of a code
span:

[Example 351](#example-351)

    *foo`*`
            

    <p>*foo<code>*</code></p>
            

And this is not parsed as a link:

[Example 352](#example-352)

    [not a `link](/foo`)
            

    <p>[not a
              <code>link](/foo</code>)</p>
            

Code spans, HTML tags, and autolinks have the same precedence. Thus,
this is code:

[Example 353](#example-353)

    `<a href="`">`
            

    <p><code>&lt;a
              href=&quot;</code>&quot;&gt;`</p>
            

But this is an HTML tag:

[Example 354](#example-354)

    <a href="`">`
            

    <p><a href="`">`</p>
            

And this is code:

[Example 355](#example-355)

    `<http://foo.bar.`baz>`
            

    <p><code>&lt;http://foo.bar.</code>baz&gt;`</p>
            

But this is an autolink:

[Example 356](#example-356)

    <http://foo.bar.`baz>`
            

    <p><a
              href="http://foo.bar.%60baz">http://foo.bar.`baz</a>`</p>
            

When a backtick string is not closed by a matching backtick string, we
just have literal backticks:

[Example 357](#example-357)

    ```foo``
            

    <p>```foo``</p>
            

[Example 358](#example-358)

    `foo
            

    <p>`foo</p>
            

The following case also illustrates the need for opening and closing
backtick strings to be equal in length:

[Example 359](#example-359)

    `foo``bar``
            

    <p>`foo<code>bar</code></p>
            

## <a href="#TOC" class="toc-link"></a><span class="number">6.4</span>Emphasis and strong emphasis

John Gruber’s original [Markdown syntax
description](http://daringfireball.net/projects/markdown/syntax#em)
says:

> Markdown treats asterisks (`*`) and underscores (`_`) as indicators of
> emphasis. Text wrapped with one `*` or `_` will be wrapped with an
> HTML `<em>` tag; double `*`’s or `_`’s will be wrapped with an HTML
> `<strong>` tag.

This is enough for most users, but these rules leave much undecided,
especially when it comes to nested emphasis. The original `Markdown.pl`
test suite makes it clear that triple `***` and `___` delimiters can be
used for strong emphasis, and most implementations have also allowed the
following patterns:

    ***strong emph***
          ***strong** in emph*
          ***emph* in strong**
          **in strong *emph***
          *in emph **strong***
        

The following patterns are less widely supported, but the intent is
clear and they are useful (especially in contexts like bibliography
entries):

    *emph *with emph* in it*
          **strong **with strong** in it**
        

Many implementations have also restricted intraword emphasis to the `*`
forms, to avoid unwanted emphasis in words containing internal
underscores. (It is best practice to put these in code spans, but users
often do not.)

    internal emphasis: foo*bar*baz
          no emphasis: foo_bar_baz
        

The rules given below capture all of these patterns, while allowing for
efficient parsing strategies that do not backtrack.

First, some definitions. A
<a href="#delimiter-run" id="delimiter-run" class="definition">delimiter
run</a> is either a sequence of one or more `*` characters that is not
preceded or followed by a non-backslash-escaped `*` character, or a
sequence of one or more `_` characters that is not preceded or followed
by a non-backslash-escaped `_` character.

A
<a href="#left-flanking-delimiter-run" id="left-flanking-delimiter-run"
class="definition">left-flanking delimiter run</a> is a [delimiter
run](#delimiter-run) that is (1) not followed by [Unicode
whitespace](#unicode-whitespace), and either (2a) not followed by a
[punctuation character](#punctuation-character), or (2b) followed by a
[punctuation character](#punctuation-character) and preceded by [Unicode
whitespace](#unicode-whitespace) or a [punctuation
character](#punctuation-character). For purposes of this definition, the
beginning and the end of the line count as Unicode whitespace.

A <a href="#right-flanking-delimiter-run"
id="right-flanking-delimiter-run" class="definition">right-flanking
delimiter run</a> is a [delimiter run](#delimiter-run) that is (1) not
preceded by [Unicode whitespace](#unicode-whitespace), and either (2a)
not preceded by a [punctuation character](#punctuation-character), or
(2b) preceded by a [punctuation character](#punctuation-character) and
followed by [Unicode whitespace](#unicode-whitespace) or a [punctuation
character](#punctuation-character). For purposes of this definition, the
beginning and the end of the line count as Unicode whitespace.

Here are some examples of delimiter runs.

- left-flanking but not right-flanking:

      ***abc
                _abc
                **"abc"
                _"abc"
              

- right-flanking but not left-flanking:

       abc***
                abc_
                "abc"**
                "abc"_
              

- Both left and right-flanking:

       abc***def
                "abc"_"def"
              

- Neither left nor right-flanking:

      abc *** def
                a _ b
              

(The idea of distinguishing left-flanking and right-flanking delimiter
runs based on the character before and the character after comes from
Roopesh Chander’s
[vfmd](http://www.vfmd.org/vfmd-spec/specification/#procedure-for-identifying-emphasis-tags).
vfmd uses the terminology “emphasis indicator string” instead of
“delimiter run,” and its rules for distinguishing left- and
right-flanking runs are a bit more complex than the ones given here.)

The following rules define emphasis and strong emphasis:

1.  A single `*` character
    <a href="#can-open-emphasis" id="can-open-emphasis"
    class="definition">can open emphasis</a> iff (if and only if) it is
    part of a [left-flanking delimiter
    run](#left-flanking-delimiter-run).

2.  A single `_` character [can open emphasis](#can-open-emphasis) iff
    it is part of a [left-flanking delimiter
    run](#left-flanking-delimiter-run) and either (a) not part of a
    [right-flanking delimiter run](#right-flanking-delimiter-run) or (b)
    part of a [right-flanking delimiter
    run](#right-flanking-delimiter-run) preceded by punctuation.

3.  A single `*` character
    <a href="#can-close-emphasis" id="can-close-emphasis"
    class="definition">can close emphasis</a> iff it is part of a
    [right-flanking delimiter run](#right-flanking-delimiter-run).

4.  A single `_` character [can close emphasis](#can-close-emphasis) iff
    it is part of a [right-flanking delimiter
    run](#right-flanking-delimiter-run) and either (a) not part of a
    [left-flanking delimiter run](#left-flanking-delimiter-run) or (b)
    part of a [left-flanking delimiter
    run](#left-flanking-delimiter-run) followed by punctuation.

5.  A double `**`
    <a href="#can-open-strong-emphasis" id="can-open-strong-emphasis"
    class="definition">can open strong emphasis</a> iff it is part of a
    [left-flanking delimiter run](#left-flanking-delimiter-run).

6.  A double `__` [can open strong emphasis](#can-open-strong-emphasis)
    iff it is part of a [left-flanking delimiter
    run](#left-flanking-delimiter-run) and either (a) not part of a
    [right-flanking delimiter run](#right-flanking-delimiter-run) or (b)
    part of a [right-flanking delimiter
    run](#right-flanking-delimiter-run) preceded by punctuation.

7.  A double `**`
    <a href="#can-close-strong-emphasis" id="can-close-strong-emphasis"
    class="definition">can close strong emphasis</a> iff it is part of a
    [right-flanking delimiter run](#right-flanking-delimiter-run).

8.  A double `__` [can close strong
    emphasis](#can-close-strong-emphasis) iff it is part of a
    [right-flanking delimiter run](#right-flanking-delimiter-run) and
    either (a) not part of a [left-flanking delimiter
    run](#left-flanking-delimiter-run) or (b) part of a [left-flanking
    delimiter run](#left-flanking-delimiter-run) followed by
    punctuation.

9.  Emphasis begins with a delimiter that [can open
    emphasis](#can-open-emphasis) and ends with a delimiter that [can
    close emphasis](#can-close-emphasis), and that uses the same
    character (`_` or `*`) as the opening delimiter. The opening and
    closing delimiters must belong to separate [delimiter
    runs](#delimiter-run). If one of the delimiters can both open and
    close emphasis, then the sum of the lengths of the delimiter runs
    containing the opening and closing delimiters must not be a multiple
    of 3 unless both lengths are multiples of 3.

10. Strong emphasis begins with a delimiter that [can open strong
    emphasis](#can-open-strong-emphasis) and ends with a delimiter that
    [can close strong emphasis](#can-close-strong-emphasis), and that
    uses the same character (`_` or `*`) as the opening delimiter. The
    opening and closing delimiters must belong to separate [delimiter
    runs](#delimiter-run). If one of the delimiters can both open and
    close strong emphasis, then the sum of the lengths of the delimiter
    runs containing the opening and closing delimiters must not be a
    multiple of 3 unless both lengths are multiples of 3.

11. A literal `*` character cannot occur at the beginning or end of
    `*`-delimited emphasis or `**`-delimited strong emphasis, unless it
    is backslash-escaped.

12. A literal `_` character cannot occur at the beginning or end of
    `_`-delimited emphasis or `__`-delimited strong emphasis, unless it
    is backslash-escaped.

Where rules 1–12 above are compatible with multiple parsings, the
following principles resolve ambiguity:

1.  The number of nestings should be minimized. Thus, for example, an
    interpretation `<strong>...</strong>` is always preferred to
    `<em><em>...</em></em>`.

2.  An interpretation `<em><strong>...</strong></em>` is always
    preferred to `<strong><em>...</em></strong>`.

3.  When two potential emphasis or strong emphasis spans overlap, so
    that the second begins before the first ends and ends after the
    first ends, the first takes precedence. Thus, for example,
    `*foo _bar* baz_` is parsed as `<em>foo _bar</em> baz_` rather than
    `*foo <em>bar* baz</em>`.

4.  When there are two potential emphasis or strong emphasis spans with
    the same closing delimiter, the shorter one (the one that opens
    later) takes precedence. Thus, for example, `**foo **bar baz**` is
    parsed as `**foo <strong>bar baz</strong>` rather than
    `<strong>foo **bar baz</strong>`.

5.  Inline code spans, links, images, and HTML tags group more tightly
    than emphasis. So, when there is a choice between an interpretation
    that contains one of these elements and one that does not, the
    former always wins. Thus, for example, `*[foo*](bar)` is parsed as
    `*<a href="bar">foo*</a>` rather than as `<em>[foo</em>](bar)`.

These rules can be illustrated through a series of examples.

Rule 1:

[Example 360](#example-360)

    *foo bar*
            

    <p><em>foo bar</em></p>
            

This is not emphasis, because the opening `*` is followed by whitespace,
and hence not part of a [left-flanking delimiter
run](#left-flanking-delimiter-run):

[Example 361](#example-361)

    a * foo bar*
            

    <p>a * foo bar*</p>
            

This is not emphasis, because the opening `*` is preceded by an
alphanumeric and followed by punctuation, and hence not part of a
[left-flanking delimiter run](#left-flanking-delimiter-run):

[Example 362](#example-362)

    a*"foo"*
            

    <p>a*&quot;foo&quot;*</p>
            

Unicode nonbreaking spaces count as whitespace, too:

[Example 363](#example-363)

    * a *
            

    <p>* a *</p>
            

Intraword emphasis with `*` is permitted:

[Example 364](#example-364)

    foo*bar*
            

    <p>foo<em>bar</em></p>
            

[Example 365](#example-365)

    5*6*78
            

    <p>5<em>6</em>78</p>
            

Rule 2:

[Example 366](#example-366)

    _foo bar_
            

    <p><em>foo bar</em></p>
            

This is not emphasis, because the opening `_` is followed by whitespace:

[Example 367](#example-367)

    _ foo bar_
            

    <p>_ foo
              bar_</p>
            

This is not emphasis, because the opening `_` is preceded by an
alphanumeric and followed by punctuation:

[Example 368](#example-368)

    a_"foo"_
            

    <p>a_&quot;foo&quot;_</p>
            

Emphasis with `_` is not allowed inside words:

[Example 369](#example-369)

    foo_bar_
            

    <p>foo_bar_</p>
            

[Example 370](#example-370)

    5_6_78
            

    <p>5_6_78</p>
            

[Example 371](#example-371)

    пристаням_стремятся_
            

    <p>пристаням_стремятся_</p>
            

Here `_` does not generate emphasis, because the first delimiter run is
right-flanking and the second left-flanking:

[Example 372](#example-372)

    aa_"bb"_cc
            

    <p>aa_&quot;bb&quot;_cc</p>
            

This is emphasis, even though the opening delimiter is both left- and
right-flanking, because it is preceded by punctuation:

[Example 373](#example-373)

    foo-_(bar)_
            

    <p>foo-<em>(bar)</em></p>
            

Rule 3:

This is not emphasis, because the closing delimiter does not match the
opening delimiter:

[Example 374](#example-374)

    _foo*
            

    <p>_foo*</p>
            

This is not emphasis, because the closing `*` is preceded by whitespace:

[Example 375](#example-375)

    *foo bar *
            

    <p>*foo bar
              *</p>
            

A newline also counts as whitespace:

[Example 376](#example-376)

    *foo bar
              *
            

    <p>*foo bar
              *</p>
            

This is not emphasis, because the second `*` is preceded by punctuation
and followed by an alphanumeric (hence it is not part of a
[right-flanking delimiter run](#right-flanking-delimiter-run):

[Example 377](#example-377)

    *(*foo)
            

    <p>*(*foo)</p>
            

The point of this restriction is more easily appreciated with this
example:

[Example 378](#example-378)

    *(*foo*)*
            

    <p><em>(<em>foo</em>)</em></p>
            

Intraword emphasis with `*` is allowed:

[Example 379](#example-379)

    *foo*bar
            

    <p><em>foo</em>bar</p>
            

Rule 4:

This is not emphasis, because the closing `_` is preceded by whitespace:

[Example 380](#example-380)

    _foo bar _
            

    <p>_foo bar
              _</p>
            

This is not emphasis, because the second `_` is preceded by punctuation
and followed by an alphanumeric:

[Example 381](#example-381)

    _(_foo)
            

    <p>_(_foo)</p>
            

This is emphasis within emphasis:

[Example 382](#example-382)

    _(_foo_)_
            

    <p><em>(<em>foo</em>)</em></p>
            

Intraword emphasis is disallowed for `_`:

[Example 383](#example-383)

    _foo_bar
            

    <p>_foo_bar</p>
            

[Example 384](#example-384)

    _пристаням_стремятся
            

    <p>_пристаням_стремятся</p>
            

[Example 385](#example-385)

    _foo_bar_baz_
            

    <p><em>foo_bar_baz</em></p>
            

This is emphasis, even though the closing delimiter is both left- and
right-flanking, because it is followed by punctuation:

[Example 386](#example-386)

    _(bar)_.
            

    <p><em>(bar)</em>.</p>
            

Rule 5:

[Example 387](#example-387)

    **foo bar**
            

    <p><strong>foo
              bar</strong></p>
            

This is not strong emphasis, because the opening delimiter is followed
by whitespace:

[Example 388](#example-388)

    ** foo bar**
            

    <p>** foo
              bar**</p>
            

This is not strong emphasis, because the opening `**` is preceded by an
alphanumeric and followed by punctuation, and hence not part of a
[left-flanking delimiter run](#left-flanking-delimiter-run):

[Example 389](#example-389)

    a**"foo"**
            

    <p>a**&quot;foo&quot;**</p>
            

Intraword strong emphasis with `**` is permitted:

[Example 390](#example-390)

    foo**bar**
            

    <p>foo<strong>bar</strong></p>
            

Rule 6:

[Example 391](#example-391)

    __foo bar__
            

    <p><strong>foo
              bar</strong></p>
            

This is not strong emphasis, because the opening delimiter is followed
by whitespace:

[Example 392](#example-392)

    __ foo bar__
            

    <p>__ foo
              bar__</p>
            

A newline counts as whitespace:

[Example 393](#example-393)

    __
              foo bar__
            

    <p>__
              foo bar__</p>
            

This is not strong emphasis, because the opening `__` is preceded by an
alphanumeric and followed by punctuation:

[Example 394](#example-394)

    a__"foo"__
            

    <p>a__&quot;foo&quot;__</p>
            

Intraword strong emphasis is forbidden with `__`:

[Example 395](#example-395)

    foo__bar__
            

    <p>foo__bar__</p>
            

[Example 396](#example-396)

    5__6__78
            

    <p>5__6__78</p>
            

[Example 397](#example-397)

    пристаням__стремятся__
            

    <p>пристаням__стремятся__</p>
            

[Example 398](#example-398)

    __foo, __bar__, baz__
            

    <p><strong>foo,
              <strong>bar</strong>, baz</strong></p>
            

This is strong emphasis, even though the opening delimiter is both left-
and right-flanking, because it is preceded by punctuation:

[Example 399](#example-399)

    foo-__(bar)__
            

    <p>foo-<strong>(bar)</strong></p>
            

Rule 7:

This is not strong emphasis, because the closing delimiter is preceded
by whitespace:

[Example 400](#example-400)

    **foo bar **
            

    <p>**foo bar
              **</p>
            

(Nor can it be interpreted as an emphasized `*foo bar *`, because of
Rule 11.)

This is not strong emphasis, because the second `**` is preceded by
punctuation and followed by an alphanumeric:

[Example 401](#example-401)

    **(**foo)
            

    <p>**(**foo)</p>
            

The point of this restriction is more easily appreciated with these
examples:

[Example 402](#example-402)

    *(**foo**)*
            

    <p><em>(<strong>foo</strong>)</em></p>
            

[Example 403](#example-403)

    **Gomphocarpus (*Gomphocarpus
              physocarpus*, syn.
              *Asclepias physocarpa*)**
            

    <p><strong>Gomphocarpus
              (<em>Gomphocarpus physocarpus</em>,
              syn.
              <em>Asclepias physocarpa</em>)</strong></p>
            

[Example 404](#example-404)

    **foo "*bar*"
              foo**
            

    <p><strong>foo
              &quot;<em>bar</em>&quot; foo</strong></p>
            

Intraword emphasis:

[Example 405](#example-405)

    **foo**bar
            

    <p><strong>foo</strong>bar</p>
            

Rule 8:

This is not strong emphasis, because the closing delimiter is preceded
by whitespace:

[Example 406](#example-406)

    __foo bar __
            

    <p>__foo bar
              __</p>
            

This is not strong emphasis, because the second `__` is preceded by
punctuation and followed by an alphanumeric:

[Example 407](#example-407)

    __(__foo)
            

    <p>__(__foo)</p>
            

The point of this restriction is more easily appreciated with this
example:

[Example 408](#example-408)

    _(__foo__)_
            

    <p><em>(<strong>foo</strong>)</em></p>
            

Intraword strong emphasis is forbidden with `__`:

[Example 409](#example-409)

    __foo__bar
            

    <p>__foo__bar</p>
            

[Example 410](#example-410)

    __пристаням__стремятся
            

    <p>__пристаням__стремятся</p>
            

[Example 411](#example-411)

    __foo__bar__baz__
            

    <p><strong>foo__bar__baz</strong></p>
            

This is strong emphasis, even though the closing delimiter is both left-
and right-flanking, because it is followed by punctuation:

[Example 412](#example-412)

    __(bar)__.
            

    <p><strong>(bar)</strong>.</p>
            

Rule 9:

Any nonempty sequence of inline elements can be the contents of an
emphasized span.

[Example 413](#example-413)

    *foo [bar](/url)*
            

    <p><em>foo <a
              href="/url">bar</a></em></p>
            

[Example 414](#example-414)

    *foo
              bar*
            

    <p><em>foo
              bar</em></p>
            

In particular, emphasis and strong emphasis can be nested inside
emphasis:

[Example 415](#example-415)

    _foo __bar__ baz_
            

    <p><em>foo
              <strong>bar</strong> baz</em></p>
            

[Example 416](#example-416)

    _foo _bar_ baz_
            

    <p><em>foo <em>bar</em> baz</em></p>
            

[Example 417](#example-417)

    __foo_ bar_
            

    <p><em><em>foo</em>
              bar</em></p>
            

[Example 418](#example-418)

    *foo *bar**
            

    <p><em>foo
              <em>bar</em></em></p>
            

[Example 419](#example-419)

    *foo **bar** baz*
            

    <p><em>foo
              <strong>bar</strong> baz</em></p>
            

[Example 420](#example-420)

    *foo**bar**baz*
            

    <p><em>foo<strong>bar</strong>baz</em></p>
            

Note that in the preceding case, the interpretation

    <p><em>foo</em><em>bar<em></em>baz</em></p>
        

is precluded by the condition that a delimiter that can both open and
close (like the `*` after `foo`) cannot form emphasis if the sum of the
lengths of the delimiter runs containing the opening and closing
delimiters is a multiple of 3 unless both lengths are multiples of 3.

For the same reason, we don’t get two consecutive emphasis sections in
this example:

[Example 421](#example-421)

    *foo**bar*
            

    <p><em>foo**bar</em></p>
            

The same condition ensures that the following cases are all strong
emphasis nested inside emphasis, even when the interior spaces are
omitted:

[Example 422](#example-422)

    ***foo** bar*
            

    <p><em><strong>foo</strong>
              bar</em></p>
            

[Example 423](#example-423)

    *foo **bar***
            

    <p><em>foo
              <strong>bar</strong></em></p>
            

[Example 424](#example-424)

    *foo**bar***
            

    <p><em>foo<strong>bar</strong></em></p>
            

When the lengths of the interior closing and opening delimiter runs are
*both* multiples of 3, though, they can match to create emphasis:

[Example 425](#example-425)

    foo***bar***baz
            

    <p>foo<em><strong>bar</strong></em>baz</p>
            

[Example 426](#example-426)

    foo******bar*********baz
            

    <p>foo<strong><strong><strong>bar</strong></strong></strong>***baz</p>
            

Indefinite levels of nesting are possible:

[Example 427](#example-427)

    *foo **bar *baz* bim** bop*
            

    <p><em>foo <strong>bar <em>baz</em> bim</strong> bop</em></p>
            

[Example 428](#example-428)

    *foo [*bar*](/url)*
            

    <p><em>foo <a
              href="/url"><em>bar</em></a></em></p>
            

There can be no empty emphasis or strong emphasis:

[Example 429](#example-429)

    ** is not an empty emphasis
            

    <p>** is not an empty emphasis</p>
            

[Example 430](#example-430)

    **** is not an empty strong emphasis
            

    <p>**** is not an empty strong emphasis</p>
            

Rule 10:

Any nonempty sequence of inline elements can be the contents of an
strongly emphasized span.

[Example 431](#example-431)

    **foo [bar](/url)**
            

    <p><strong>foo <a
              href="/url">bar</a></strong></p>
            

[Example 432](#example-432)

    **foo
              bar**
            

    <p><strong>foo
              bar</strong></p>
            

In particular, emphasis and strong emphasis can be nested inside strong
emphasis:

[Example 433](#example-433)

    __foo _bar_ baz__
            

    <p><strong>foo
              <em>bar</em> baz</strong></p>
            

[Example 434](#example-434)

    __foo __bar__ baz__
            

    <p><strong>foo
              <strong>bar</strong> baz</strong></p>
            

[Example 435](#example-435)

    ____foo__ bar__
            

    <p><strong><strong>foo</strong>
              bar</strong></p>
            

[Example 436](#example-436)

    **foo **bar****
            

    <p><strong>foo
              <strong>bar</strong></strong></p>
            

[Example 437](#example-437)

    **foo *bar* baz**
            

    <p><strong>foo
              <em>bar</em> baz</strong></p>
            

[Example 438](#example-438)

    **foo*bar*baz**
            

    <p><strong>foo<em>bar</em>baz</strong></p>
            

[Example 439](#example-439)

    ***foo* bar**
            

    <p><strong><em>foo</em>
              bar</strong></p>
            

[Example 440](#example-440)

    **foo *bar***
            

    <p><strong>foo
              <em>bar</em></strong></p>
            

Indefinite levels of nesting are possible:

[Example 441](#example-441)

    **foo *bar **baz**
              bim* bop**
            

    <p><strong>foo <em>bar <strong>baz</strong>
              bim</em> bop</strong></p>
            

[Example 442](#example-442)

    **foo [*bar*](/url)**
            

    <p><strong>foo <a
              href="/url"><em>bar</em></a></strong></p>
            

There can be no empty emphasis or strong emphasis:

[Example 443](#example-443)

    __ is not an empty emphasis
            

    <p>__ is not an empty emphasis</p>
            

[Example 444](#example-444)

    ____ is not an empty strong emphasis
            

    <p>____ is not an empty strong emphasis</p>
            

Rule 11:

[Example 445](#example-445)

    foo ***
            

    <p>foo ***</p>
            

[Example 446](#example-446)

    foo *\**
            

    <p>foo <em>*</em></p>
            

[Example 447](#example-447)

    foo *_*
            

    <p>foo <em>_</em></p>
            

[Example 448](#example-448)

    foo *****
            

    <p>foo *****</p>
            

[Example 449](#example-449)

    foo **\***
            

    <p>foo <strong>*</strong></p>
            

[Example 450](#example-450)

    foo **_**
            

    <p>foo <strong>_</strong></p>
            

Note that when delimiters do not match evenly, Rule 11 determines that
the excess literal `*` characters will appear outside of the emphasis,
rather than inside it:

[Example 451](#example-451)

    **foo*
            

    <p>*<em>foo</em></p>
            

[Example 452](#example-452)

    *foo**
            

    <p><em>foo</em>*</p>
            

[Example 453](#example-453)

    ***foo**
            

    <p>*<strong>foo</strong></p>
            

[Example 454](#example-454)

    ****foo*
            

    <p>***<em>foo</em></p>
            

[Example 455](#example-455)

    **foo***
            

    <p><strong>foo</strong>*</p>
            

[Example 456](#example-456)

    *foo****
            

    <p><em>foo</em>***</p>
            

Rule 12:

[Example 457](#example-457)

    foo ___
            

    <p>foo ___</p>
            

[Example 458](#example-458)

    foo _\__
            

    <p>foo <em>_</em></p>
            

[Example 459](#example-459)

    foo _*_
            

    <p>foo <em>*</em></p>
            

[Example 460](#example-460)

    foo _____
            

    <p>foo _____</p>
            

[Example 461](#example-461)

    foo __\___
            

    <p>foo <strong>_</strong></p>
            

[Example 462](#example-462)

    foo __*__
            

    <p>foo <strong>*</strong></p>
            

[Example 463](#example-463)

    __foo_
            

    <p>_<em>foo</em></p>
            

Note that when delimiters do not match evenly, Rule 12 determines that
the excess literal `_` characters will appear outside of the emphasis,
rather than inside it:

[Example 464](#example-464)

    _foo__
            

    <p><em>foo</em>_</p>
            

[Example 465](#example-465)

    ___foo__
            

    <p>_<strong>foo</strong></p>
            

[Example 466](#example-466)

    ____foo_
            

    <p>___<em>foo</em></p>
            

[Example 467](#example-467)

    __foo___
            

    <p><strong>foo</strong>_</p>
            

[Example 468](#example-468)

    _foo____
            

    <p><em>foo</em>___</p>
            

Rule 13 implies that if you want emphasis nested directly inside
emphasis, you must use different delimiters:

[Example 469](#example-469)

    **foo**
            

    <p><strong>foo</strong></p>
            

[Example 470](#example-470)

    *_foo_*
            

    <p><em><em>foo</em></em></p>
            

[Example 471](#example-471)

    __foo__
            

    <p><strong>foo</strong></p>
            

[Example 472](#example-472)

    _*foo*_
            

    <p><em><em>foo</em></em></p>
            

However, strong emphasis within strong emphasis is possible without
switching delimiters:

[Example 473](#example-473)

    ****foo****
            

    <p><strong><strong>foo</strong></strong></p>
            

[Example 474](#example-474)

    ____foo____
            

    <p><strong><strong>foo</strong></strong></p>
            

Rule 13 can be applied to arbitrarily long sequences of delimiters:

[Example 475](#example-475)

    ******foo******
            

    <p><strong><strong><strong>foo</strong></strong></strong></p>
            

Rule 14:

[Example 476](#example-476)

    ***foo***
            

    <p><em><strong>foo</strong></em></p>
            

[Example 477](#example-477)

    _____foo_____
            

    <p><em><strong><strong>foo</strong></strong></em></p>
            

Rule 15:

[Example 478](#example-478)

    *foo _bar* baz_
            

    <p><em>foo _bar</em> baz_</p>
            

[Example 479](#example-479)

    *foo __bar *baz bim__ bam*
            

    <p><em>foo <strong>bar *baz bim</strong>
              bam</em></p>
            

Rule 16:

[Example 480](#example-480)

    **foo **bar baz**
            

    <p>**foo <strong>bar
              baz</strong></p>
            

[Example 481](#example-481)

    *foo *bar baz*
            

    <p>*foo <em>bar
              baz</em></p>
            

Rule 17:

[Example 482](#example-482)

    *[bar*](/url)
            

    <p>*<a
              href="/url">bar*</a></p>
            

[Example 483](#example-483)

    _foo [bar_](/url)
            

    <p>_foo <a
              href="/url">bar_</a></p>
            

[Example 484](#example-484)

    *<img src="foo"
              title="*"/>
            

    <p>*<img src="foo" title="*"/></p>
            

[Example 485](#example-485)

    **<a href="**">
            

    <p>**<a href="**"></p>
            

[Example 486](#example-486)

    __<a href="__">
            

    <p>__<a href="__"></p>
            

[Example 487](#example-487)

    *a `*`*
            

    <p><em>a
              <code>*</code></em></p>
            

[Example 488](#example-488)

    _a `_`_
            

    <p><em>a
              <code>_</code></em></p>
            

[Example 489](#example-489)

    **a<http://foo.bar/?q=**>
            

    <p>**a<a
              href="http://foo.bar/?q=**">http://foo.bar/?q=**</a></p>
            

[Example 490](#example-490)

    __a<http://foo.bar/?q=__>
            

    <p>__a<a
              href="http://foo.bar/?q=__">http://foo.bar/?q=__</a></p>
            

## <a href="#TOC" class="toc-link"></a><span class="number">6.5</span>Strikethrough (extension)

GFM enables the `strikethrough` extension, where an additional emphasis
type is available.

Strikethrough text is any text wrapped in a matching pair of one or two
tildes (`~`).

[Example 491](#example-491)

    ~~Hi~~ Hello,
                ~there~ world!
              

    <p><del>Hi</del> Hello, <del>there</del> world!</p>
              

As with regular emphasis delimiters, a new paragraph will cause
strikethrough parsing to cease:

[Example 492](#example-492)

    This ~~has a

                new paragraph~~.
              

    <p>This ~~has
                a</p>
                <p>new paragraph~~.</p>
              

Three or more tildes do not create a strikethrough:

[Example 493](#example-493)

    This will
                ~~~not~~~ strike.
              

    <p>This will
                ~~~not~~~ strike.</p>
              

## <a href="#TOC" class="toc-link"></a><span class="number">6.6</span>Links

A link contains [link text](#link-text) (the visible text), a [link
destination](#link-destination) (the URI that is the link destination),
and optionally a [link title](#link-title). There are two basic kinds of
links in Markdown. In [inline links](#inline-link) the destination and
title are given immediately after the link text. In [reference
links](#reference-link) the destination and title are defined elsewhere
in the document.

A <a href="#link-text" id="link-text" class="definition">link text</a>
consists of a sequence of zero or more inline elements enclosed by
square brackets (`[` and `]`). The following rules apply:

- Links may not contain other links, at any level of nesting. If
  multiple otherwise valid link definitions appear nested inside each
  other, the inner-most definition is used.

- Brackets are allowed in the [link text](#link-text) only if (a) they
  are backslash-escaped or (b) they appear as a matched pair of
  brackets, with an open bracket `[`, a sequence of zero or more
  inlines, and a close bracket `]`.

- Backtick [code spans](#code-spans), [autolinks](#autolinks), and raw
  [HTML tags](#html-tag) bind more tightly than the brackets in link
  text. Thus, for example, `` [foo`]` `` could not be a link text, since
  the second `]` is part of a code span.

- The brackets in link text bind more tightly than markers for [emphasis
  and strong emphasis](#emphasis-and-strong-emphasis). Thus, for
  example, `*[foo*](url)` is a link.

A <a href="#link-destination" id="link-destination"
class="definition">link destination</a> consists of either

- a sequence of zero or more characters between an opening `<` and a
  closing `>` that contains no line breaks or unescaped `<` or `>`
  characters, or

- a nonempty sequence of characters that does not start with `<`, does
  not include ASCII space or control characters, and includes
  parentheses only if (a) they are backslash-escaped or (b) they are
  part of a balanced pair of unescaped parentheses. (Implementations may
  impose limits on parentheses nesting to avoid performance issues, but
  at least three levels of nesting should be supported.)

A
<a href="#link-title" id="link-title" class="definition">link title</a>
consists of either

- a sequence of zero or more characters between straight double-quote
  characters (`"`), including a `"` character only if it is
  backslash-escaped, or

- a sequence of zero or more characters between straight single-quote
  characters (`'`), including a `'` character only if it is
  backslash-escaped, or

- a sequence of zero or more characters between matching parentheses
  (`(...)`), including a `(` or `)` character only if it is
  backslash-escaped.

Although [link titles](#link-title) may span multiple lines, they may
not contain a [blank line](#blank-line).

An <a href="#inline-link" id="inline-link" class="definition">inline
link</a> consists of a [link text](#link-text) followed immediately by a
left parenthesis `(`, optional [whitespace](#whitespace), an optional
[link destination](#link-destination), an optional [link
title](#link-title) separated from the link destination by
[whitespace](#whitespace), optional [whitespace](#whitespace), and a
right parenthesis `)`. The link’s text consists of the inlines contained
in the [link text](#link-text) (excluding the enclosing square
brackets). The link’s URI consists of the link destination, excluding
enclosing `<...>` if present, with backslash-escapes in effect as
described above. The link’s title consists of the link title, excluding
its enclosing delimiters, with backslash-escapes in effect as described
above.

Here is a simple inline link:

[Example 494](#example-494)

    [link](/uri "title")
            

    <p><a href="/uri" title="title">link</a></p>
            

The title may be omitted:

[Example 495](#example-495)

    [link](/uri)
            

    <p><a
              href="/uri">link</a></p>
            

Both the title and the destination may be omitted:

[Example 496](#example-496)

    [link]()
            

    <p><a
              href="">link</a></p>
            

[Example 497](#example-497)

    [link](<>)
            

    <p><a
              href="">link</a></p>
            

The destination can only contain spaces if it is enclosed in pointy
brackets:

[Example 498](#example-498)

    [link](/my uri)
            

    <p>[link](/my uri)</p>
            

[Example 499](#example-499)

    [link](</my uri>)
            

    <p><a
              href="/my%20uri">link</a></p>
            

The destination cannot contain line breaks, even if enclosed in pointy
brackets:

[Example 500](#example-500)

    [link](foo
              bar)
            

    <p>[link](foo
              bar)</p>
            

[Example 501](#example-501)

    [link](<foo
              bar>)
            

    <p>[link](<foo
              bar>)</p>
            

The destination can contain `)` if it is enclosed in pointy brackets:

[Example 502](#example-502)

    [a](<b)c>)
            

    <p><a
              href="b)c">a</a></p>
            

Pointy brackets that enclose links must be unescaped:

[Example 503](#example-503)

    [link](<foo\>)
            

    <p>[link](&lt;foo&gt;)</p>
            

These are not links, because the opening pointy bracket is not matched
properly:

[Example 504](#example-504)

    [a](<b)c
              [a](<b)c>
              [a](<b>c)
            

    <p>[a](&lt;b)c
              [a](&lt;b)c&gt;
              [a](<b>c)</p>
            

Parentheses inside the link destination may be escaped:

[Example 505](#example-505)

    [link](\(foo\))
            

    <p><a
              href="(foo)">link</a></p>
            

Any number of parentheses are allowed without escaping, as long as they
are balanced:

[Example 506](#example-506)

    [link](foo(and(bar)))
            

    <p><a
              href="foo(and(bar))">link</a></p>
            

However, if you have unbalanced parentheses, you need to escape or use
the `<...>` form:

[Example 507](#example-507)

    [link](foo\(and\(bar\))
            

    <p><a
              href="foo(and(bar)">link</a></p>
            

[Example 508](#example-508)

    [link](<foo(and(bar)>)
            

    <p><a
              href="foo(and(bar)">link</a></p>
            

Parentheses and other symbols can also be escaped, as usual in Markdown:

[Example 509](#example-509)

    [link](foo\)\:)
            

    <p><a
              href="foo):">link</a></p>
            

A link can contain fragment identifiers and queries:

[Example 510](#example-510)

    [link](#fragment)

              [link](http://example.com#fragment)

              [link](http://example.com?foo=3#frag)
            

    <p><a
              href="#fragment">link</a></p>
              <p><a
              href="http://example.com#fragment">link</a></p>
              <p><a
              href="http://example.com?foo=3#frag">link</a></p>
            

Note that a backslash before a non-escapable character is just a
backslash:

[Example 511](#example-511)

    [link](foo\bar)
            

    <p><a
              href="foo%5Cbar">link</a></p>
            

URL-escaping should be left alone inside the destination, as all
URL-escaped characters are also valid URL characters. Entity and
numerical character references in the destination will be parsed into
the corresponding Unicode code points, as usual. These may be optionally
URL-escaped when written as HTML, but this spec does not enforce any
particular policy for rendering URLs in HTML or other formats. Renderers
may make different decisions about how to escape or normalize URLs in
the output.

[Example 512](#example-512)

    [link](foo%20b&auml;)
            

    <p><a
              href="foo%20b%C3%A4">link</a></p>
            

Note that, because titles can often be parsed as destinations, if you
try to omit the destination and keep the title, you’ll get unexpected
results:

[Example 513](#example-513)

    [link]("title")
            

    <p><a
              href="%22title%22">link</a></p>
            

Titles may be in single quotes, double quotes, or parentheses:

[Example 514](#example-514)

    [link](/url "title")
              [link](/url 'title')
              [link](/url (title))
            

    <p><a href="/url" title="title">link</a>
              <a href="/url"
              title="title">link</a>
              <a href="/url"
              title="title">link</a></p>
            

Backslash escapes and entity and numeric character references may be
used in titles:

[Example 515](#example-515)

    [link](/url "title
              \"&quot;")
            

    <p><a href="/url" title="title
              &quot;&quot;">link</a></p>
            

Titles must be separated from the link using a
[whitespace](#whitespace). Other [Unicode
whitespace](#unicode-whitespace) like non-breaking space doesn’t work.

[Example 516](#example-516)

    [link](/url "title")
            

    <p><a
              href="/url%C2%A0%22title%22">link</a></p>
            

Nested balanced quotes are not allowed without escaping:

[Example 517](#example-517)

    [link](/url "title
              "and" title")
            

    <p>[link](/url &quot;title &quot;and&quot; title&quot;)</p>
            

But it is easy to work around this by using a different quote type:

[Example 518](#example-518)

    [link](/url 'title
              "and" title')
            

    <p><a href="/url" title="title &quot;and&quot; title">link</a></p>
            

(Note: `Markdown.pl` did allow double quotes inside a double-quoted
title, and its test suite included a test demonstrating this. But it is
hard to see a good rationale for the extra complexity this brings, since
there are already many ways—backslash escaping, entity and numeric
character references, or using a different quote type for the enclosing
title—to write titles containing double quotes. `Markdown.pl`’s handling
of titles has a number of other strange features. For example, it allows
single-quoted titles in inline links, but not reference links. And, in
reference links but not inline links, it allows a title to begin with
`"` and end with `)`. `Markdown.pl` 1.0.1 even allows titles with no
closing quotation mark, though 1.0.2b8 does not. It seems preferable to
adopt a simple, rational rule that works the same way in inline links
and link reference definitions.)

[Whitespace](#whitespace) is allowed around the destination and title:

[Example 519](#example-519)

    [link](   /uri
                "title"  )
            

    <p><a href="/uri" title="title">link</a></p>
            

But it is not allowed between the link text and the following
parenthesis:

[Example 520](#example-520)

    [link] (/uri)
            

    <p>[link] (/uri)</p>
            

The link text may contain balanced brackets, but not unbalanced ones,
unless they are escaped:

[Example 521](#example-521)

    [link [foo
              [bar]]](/uri)
            

    <p><a href="/uri">link [foo [bar]]</a></p>
            

[Example 522](#example-522)

    [link] bar](/uri)
            

    <p>[link] bar](/uri)</p>
            

[Example 523](#example-523)

    [link [bar](/uri)
            

    <p>[link <a
              href="/uri">bar</a></p>
            

[Example 524](#example-524)

    [link \[bar](/uri)
            

    <p><a href="/uri">link [bar</a></p>
            

The link text may contain inline content:

[Example 525](#example-525)

    [link *foo **bar** `#`*](/uri)
            

    <p><a href="/uri">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>
            

[Example 526](#example-526)

    [![moon](moon.jpg)](/uri)
            

    <p><a href="/uri"><img src="moon.jpg" alt="moon" /></a></p>
            

However, links may not contain other links, at any level of nesting.

[Example 527](#example-527)

    [foo [bar](/uri)](/uri)
            

    <p>[foo <a
              href="/uri">bar</a>](/uri)</p>
            

[Example 528](#example-528)

    [foo *[bar
              [baz](/uri)](/uri)*](/uri)
            

    <p>[foo <em>[bar
              <a
              href="/uri">baz</a>](/uri)</em>](/uri)</p>
            

[Example 529](#example-529)

    ![[[foo](uri1)](uri2)](uri3)
            

    <p><img src="uri3" alt="[foo](uri2)" /></p>
            

These cases illustrate the precedence of link text grouping over
emphasis grouping:

[Example 530](#example-530)

    *[foo*](/uri)
            

    <p>*<a
              href="/uri">foo*</a></p>
            

[Example 531](#example-531)

    [foo *bar](baz*)
            

    <p><a href="baz*">foo *bar</a></p>
            

Note that brackets that *aren’t* part of links do not take precedence:

[Example 532](#example-532)

    *foo [bar* baz]
            

    <p><em>foo [bar</em> baz]</p>
            

These cases illustrate the precedence of HTML tags, code spans, and
autolinks over link grouping:

[Example 533](#example-533)

    [foo <bar
              attr="](baz)">
            

    <p>[foo <bar
              attr="](baz)"></p>
            

[Example 534](#example-534)

    [foo`](/uri)`
            

    <p>[foo<code>](/uri)</code></p>
            

[Example 535](#example-535)

    [foo<http://example.com/?search=](uri)>
            

    <p>[foo<a
              href="http://example.com/?search=%5D(uri)">http://example.com/?search=](uri)</a></p>
            

There are three kinds of <a href="#reference-link" id="reference-link"
class="definition">reference link</a>s: [full](#full-reference-link),
[collapsed](#collapsed-reference-link), and
[shortcut](#shortcut-reference-link).

A <a href="#full-reference-link" id="full-reference-link"
class="definition">full reference link</a> consists of a [link
text](#link-text) immediately followed by a [link label](#link-label)
that [matches](#matches) a [link reference
definition](#link-reference-definition) elsewhere in the document.

A
<a href="#link-label" id="link-label" class="definition">link label</a>
begins with a left bracket (`[`) and ends with the first right bracket
(`]`) that is not backslash-escaped. Between these brackets there must
be at least one [non-whitespace character](#non-whitespace-character).
Unescaped square bracket characters are not allowed inside the opening
and closing square brackets of [link labels](#link-label). A link label
can have at most 999 characters inside the square brackets.

One label <a href="#matches" id="matches" class="definition">matches</a>
another just in case their normalized forms are equal. To normalize a
label, strip off the opening and closing brackets, perform the *Unicode
case fold*, strip leading and trailing [whitespace](#whitespace) and
collapse consecutive internal [whitespace](#whitespace) to a single
space. If there are multiple matching reference link definitions, the
one that comes first in the document is used. (It is desirable in such
cases to emit a warning.)

The link’s URI and title are provided by the matching [link reference
definition](#link-reference-definition).

Here is a simple example:

[Example 536](#example-536)

    [foo][bar]

              [bar]: /url "title"
            

    <p><a href="/url" title="title">foo</a></p>
            

The rules for the [link text](#link-text) are the same as with [inline
links](#inline-link). Thus:

The link text may contain balanced brackets, but not unbalanced ones,
unless they are escaped:

[Example 537](#example-537)

    [link [foo [bar]]][ref]

              [ref]: /uri
            

    <p><a href="/uri">link [foo [bar]]</a></p>
            

[Example 538](#example-538)

    [link \[bar][ref]

              [ref]: /uri
            

    <p><a href="/uri">link [bar</a></p>
            

The link text may contain inline content:

[Example 539](#example-539)

    [link *foo **bar** `#`*][ref]

              [ref]: /uri
            

    <p><a href="/uri">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>
            

[Example 540](#example-540)

    [![moon](moon.jpg)][ref]

              [ref]: /uri
            

    <p><a href="/uri"><img src="moon.jpg" alt="moon" /></a></p>
            

However, links may not contain other links, at any level of nesting.

[Example 541](#example-541)

    [foo [bar](/uri)][ref]

              [ref]: /uri
            

    <p>[foo <a
              href="/uri">bar</a>]<a
              href="/uri">ref</a></p>
            

[Example 542](#example-542)

    [foo *bar
              [baz][ref]*][ref]

              [ref]: /uri
            

    <p>[foo <em>bar
              <a href="/uri">baz</a></em>]<a href="/uri">ref</a></p>
            

(In the examples above, we have two [shortcut reference
links](#shortcut-reference-link) instead of one [full reference
link](#full-reference-link).)

The following cases illustrate the precedence of link text grouping over
emphasis grouping:

[Example 543](#example-543)

    *[foo*][ref]

              [ref]: /uri
            

    <p>*<a
              href="/uri">foo*</a></p>
            

[Example 544](#example-544)

    [foo *bar][ref]*

              [ref]: /uri
            

    <p><a href="/uri">foo *bar</a>*</p>
            

These cases illustrate the precedence of HTML tags, code spans, and
autolinks over link grouping:

[Example 545](#example-545)

    [foo <bar
              attr="][ref]">

              [ref]: /uri
            

    <p>[foo <bar
              attr="][ref]"></p>
            

[Example 546](#example-546)

    [foo`][ref]`

              [ref]: /uri
            

    <p>[foo<code>][ref]</code></p>
            

[Example 547](#example-547)

    [foo<http://example.com/?search=][ref]>

              [ref]: /uri
            

    <p>[foo<a
              href="http://example.com/?search=%5D%5Bref%5D">http://example.com/?search=][ref]</a></p>
            

Matching is case-insensitive:

[Example 548](#example-548)

    [foo][BaR]

              [bar]: /url "title"
            

    <p><a href="/url" title="title">foo</a></p>
            

Unicode case fold is used:

[Example 549](#example-549)

    [ẞ]

              [SS]: /url
            

    <p><a
              href="/url">ẞ</a></p>
            

Consecutive internal [whitespace](#whitespace) is treated as one space
for purposes of determining matching:

[Example 550](#example-550)

    [Foo
                bar]: /url

              [Baz][Foo bar]
            

    <p><a
              href="/url">Baz</a></p>
            

No [whitespace](#whitespace) is allowed between the [link
text](#link-text) and the [link label](#link-label):

[Example 551](#example-551)

    [foo] [bar]

              [bar]: /url "title"
            

    <p>[foo] <a
              href="/url" title="title">bar</a></p>
            

[Example 552](#example-552)

    [foo]
              [bar]

              [bar]: /url "title"
            

    <p>[foo]
              <a href="/url"
              title="title">bar</a></p>
            

This is a departure from John Gruber’s original Markdown syntax
description, which explicitly allows whitespace between the link text
and the link label. It brings reference links in line with [inline
links](#inline-link), which (according to both original Markdown and
this spec) cannot have whitespace after the link text. More importantly,
it prevents inadvertent capture of consecutive [shortcut reference
links](#shortcut-reference-link). If whitespace is allowed between the
link text and the link label, then in the following we will have a
single reference link, not two shortcut reference links, as intended:

    [foo]
          [bar]

          [foo]: /url1
          [bar]: /url2
        

(Note that [shortcut reference links](#shortcut-reference-link) were
introduced by Gruber himself in a beta version of `Markdown.pl`, but
never included in the official syntax description. Without shortcut
reference links, it is harmless to allow space between the link text and
link label; but once shortcut references are introduced, it is too
dangerous to allow this, as it frequently leads to unintended results.)

When there are multiple matching [link reference
definitions](#link-reference-definitions), the first is used:

[Example 553](#example-553)

    [foo]: /url1

              [foo]: /url2

              [bar][foo]
            

    <p><a
              href="/url1">bar</a></p>
            

Note that matching is performed on normalized strings, not parsed inline
content. So the following does not match, even though the labels define
equivalent inline content:

[Example 554](#example-554)

    [bar][foo\!]

              [foo!]: /url
            

    <p>[bar][foo!]</p>
            

[Link labels](#link-label) cannot contain brackets, unless they are
backslash-escaped:

[Example 555](#example-555)

    [foo][ref[]

              [ref[]: /uri
            

    <p>[foo][ref[]</p>
              <p>[ref[]: /uri</p>
            

[Example 556](#example-556)

    [foo][ref[bar]]

              [ref[bar]]: /uri
            

    <p>[foo][ref[bar]]</p>
              <p>[ref[bar]]: /uri</p>
            

[Example 557](#example-557)

    [[[foo]]]

              [[[foo]]]: /url
            

    <p>[[[foo]]]</p>
              <p>[[[foo]]]: /url</p>
            

[Example 558](#example-558)

    [foo][ref\[]

              [ref\[]: /uri
            

    <p><a
              href="/uri">foo</a></p>
            

Note that in this example `]` is not backslash-escaped:

[Example 559](#example-559)

    [bar\\]: /uri

              [bar\\]
            

    <p><a
              href="/uri">bar\</a></p>
            

A [link label](#link-label) must contain at least one [non-whitespace
character](#non-whitespace-character):

[Example 560](#example-560)

    []

              []: /uri
            

    <p>[]</p>
              <p>[]: /uri</p>
            

[Example 561](#example-561)

    [
               ]

              [
               ]: /uri
            

    <p>[
              ]</p>
              <p>[
              ]: /uri</p>
            

A <a href="#collapsed-reference-link" id="collapsed-reference-link"
class="definition">collapsed reference link</a> consists of a [link
label](#link-label) that [matches](#matches) a [link reference
definition](#link-reference-definition) elsewhere in the document,
followed by the string `[]`. The contents of the first link label are
parsed as inlines, which are used as the link’s text. The link’s URI and
title are provided by the matching reference link definition. Thus,
`[foo][]` is equivalent to `[foo][foo]`.

[Example 562](#example-562)

    [foo][]

              [foo]: /url "title"
            

    <p><a href="/url" title="title">foo</a></p>
            

[Example 563](#example-563)

    [*foo* bar][]

              [*foo* bar]: /url
              "title"
            

    <p><a href="/url" title="title"><em>foo</em>
              bar</a></p>
            

The link labels are case-insensitive:

[Example 564](#example-564)

    [Foo][]

              [foo]: /url "title"
            

    <p><a href="/url" title="title">Foo</a></p>
            

As with full reference links, [whitespace](#whitespace) is not allowed
between the two sets of brackets:

[Example 565](#example-565)

    [foo] 
              []

              [foo]: /url "title"
            

    <p><a href="/url" title="title">foo</a>
              []</p>
            

A <a href="#shortcut-reference-link" id="shortcut-reference-link"
class="definition">shortcut reference link</a> consists of a [link
label](#link-label) that [matches](#matches) a [link reference
definition](#link-reference-definition) elsewhere in the document and is
not followed by `[]` or a link label. The contents of the first link
label are parsed as inlines, which are used as the link’s text. The
link’s URI and title are provided by the matching link reference
definition. Thus, `[foo]` is equivalent to `[foo][]`.

[Example 566](#example-566)

    [foo]

              [foo]: /url "title"
            

    <p><a href="/url" title="title">foo</a></p>
            

[Example 567](#example-567)

    [*foo* bar]

              [*foo* bar]: /url
              "title"
            

    <p><a href="/url" title="title"><em>foo</em>
              bar</a></p>
            

[Example 568](#example-568)

    [[*foo* bar]]

              [*foo* bar]: /url
              "title"
            

    <p>[<a href="/url" title="title"><em>foo</em>
              bar</a>]</p>
            

[Example 569](#example-569)

    [[bar [foo]

              [foo]: /url
            

    <p>[[bar <a
              href="/url">foo</a></p>
            

The link labels are case-insensitive:

[Example 570](#example-570)

    [Foo]

              [foo]: /url "title"
            

    <p><a href="/url" title="title">Foo</a></p>
            

A space after the link text should be preserved:

[Example 571](#example-571)

    [foo] bar

              [foo]: /url
            

    <p><a
              href="/url">foo</a> bar</p>
            

If you just want bracketed text, you can backslash-escape the opening
bracket to avoid links:

[Example 572](#example-572)

    \[foo]

              [foo]: /url "title"
            

    <p>[foo]</p>
            

Note that this is a link, because a link label ends with the first
following closing bracket:

[Example 573](#example-573)

    [foo*]: /url

              *[foo*]
            

    <p>*<a
              href="/url">foo*</a></p>
            

Full and compact references take precedence over shortcut references:

[Example 574](#example-574)

    [foo][bar]

              [foo]: /url1
              [bar]: /url2
            

    <p><a
              href="/url2">foo</a></p>
            

[Example 575](#example-575)

    [foo][]

              [foo]: /url1
            

    <p><a
              href="/url1">foo</a></p>
            

Inline links also take precedence:

[Example 576](#example-576)

    [foo]()

              [foo]: /url1
            

    <p><a
              href="">foo</a></p>
            

[Example 577](#example-577)

    [foo](not a link)

              [foo]: /url1
            

    <p><a
              href="/url1">foo</a>(not a
              link)</p>
            

In the following case `[bar][baz]` is parsed as a reference, `[foo]` as
normal text:

[Example 578](#example-578)

    [foo][bar][baz]

              [baz]: /url
            

    <p>[foo]<a
              href="/url">bar</a></p>
            

Here, though, `[foo][bar]` is parsed as a reference, since `[bar]` is
defined:

[Example 579](#example-579)

    [foo][bar][baz]

              [baz]: /url1
              [bar]: /url2
            

    <p><a
              href="/url2">foo</a><a
              href="/url1">baz</a></p>
            

Here `[foo]` is not parsed as a shortcut reference, because it is
followed by a link label (even though `[bar]` is not defined):

[Example 580](#example-580)

    [foo][bar][baz]

              [baz]: /url1
              [foo]: /url2
            

    <p>[foo]<a
              href="/url1">bar</a></p>
            

## <a href="#TOC" class="toc-link"></a><span class="number">6.7</span>Images

Syntax for images is like the syntax for links, with one difference.
Instead of [link text](#link-text), we have an
<a href="#image-description" id="image-description"
class="definition">image description</a>. The rules for this are the
same as for [link text](#link-text), except that (a) an image
description starts with `![` rather than `[`, and (b) an image
description may contain links. An image description has inline elements
as its contents. When an image is rendered to HTML, this is standardly
used as the image’s `alt` attribute.

[Example 581](#example-581)

    ![foo](/url "title")
            

    <p><img src="/url" alt="foo" title="title" /></p>
            

[Example 582](#example-582)

    ![foo *bar*]

              [foo *bar*]: train.jpg
              "train & tracks"
            

    <p><img src="train.jpg" alt="foo bar"
              title="train &amp; tracks" /></p>
            

[Example 583](#example-583)

    ![foo ![bar](/url)](/url2)
            

    <p><img src="/url2" alt="foo bar"
              /></p>
            

[Example 584](#example-584)

    ![foo [bar](/url)](/url2)
            

    <p><img src="/url2" alt="foo bar"
              /></p>
            

Though this spec is concerned with parsing, not rendering, it is
recommended that in rendering to HTML, only the plain string content of
the [image description](#image-description) be used. Note that in the
above example, the alt attribute’s value is `foo bar`, not
`foo [bar](/url)` or `foo <a href="/url">bar</a>`. Only the plain string
content is rendered, without formatting.

[Example 585](#example-585)

    ![foo *bar*][]

              [foo *bar*]: train.jpg
              "train & tracks"
            

    <p><img src="train.jpg" alt="foo bar"
              title="train &amp; tracks" /></p>
            

[Example 586](#example-586)

    ![foo *bar*][foobar]

              [FOOBAR]: train.jpg "train
              & tracks"
            

    <p><img src="train.jpg" alt="foo bar"
              title="train &amp; tracks" /></p>
            

[Example 587](#example-587)

    ![foo](train.jpg)
            

    <p><img src="train.jpg" alt="foo" /></p>
            

[Example 588](#example-588)

    My ![foo
              bar](/path/to/train.jpg  "title"   )
            

    <p>My <img
              src="/path/to/train.jpg" alt="foo
              bar" title="title" /></p>
            

[Example 589](#example-589)

    ![foo](<url>)
            

    <p><img src="url" alt="foo" /></p>
            

[Example 590](#example-590)

    ![](/url)
            

    <p><img src="/url" alt="" /></p>
            

Reference-style:

[Example 591](#example-591)

    ![foo][bar]

              [bar]: /url
            

    <p><img src="/url" alt="foo" /></p>
            

[Example 592](#example-592)

    ![foo][bar]

              [BAR]: /url
            

    <p><img src="/url" alt="foo" /></p>
            

Collapsed:

[Example 593](#example-593)

    ![foo][]

              [foo]: /url "title"
            

    <p><img src="/url" alt="foo" title="title" /></p>
            

[Example 594](#example-594)

    ![*foo* bar][]

              [*foo* bar]: /url
              "title"
            

    <p><img src="/url" alt="foo bar"
              title="title" /></p>
            

The labels are case-insensitive:

[Example 595](#example-595)

    ![Foo][]

              [foo]: /url "title"
            

    <p><img src="/url" alt="Foo" title="title" /></p>
            

As with reference links, [whitespace](#whitespace) is not allowed
between the two sets of brackets:

[Example 596](#example-596)

    ![foo] 
              []

              [foo]: /url "title"
            

    <p><img src="/url" alt="foo" title="title" />
              []</p>
            

Shortcut:

[Example 597](#example-597)

    ![foo]

              [foo]: /url "title"
            

    <p><img src="/url" alt="foo" title="title" /></p>
            

[Example 598](#example-598)

    ![*foo* bar]

              [*foo* bar]: /url
              "title"
            

    <p><img src="/url" alt="foo bar"
              title="title" /></p>
            

Note that link labels cannot contain unescaped brackets:

[Example 599](#example-599)

    ![[foo]]

              [[foo]]: /url "title"
            

    <p>![[foo]]</p>
              <p>[[foo]]: /url
              &quot;title&quot;</p>
            

The link labels are case-insensitive:

[Example 600](#example-600)

    ![Foo]

              [foo]: /url "title"
            

    <p><img src="/url" alt="Foo" title="title" /></p>
            

If you just want a literal `!` followed by bracketed text, you can
backslash-escape the opening `[`:

[Example 601](#example-601)

    !\[foo]

              [foo]: /url "title"
            

    <p>![foo]</p>
            

If you want a link after a literal `!`, backslash-escape the `!`:

[Example 602](#example-602)

    \![foo]

              [foo]: /url "title"
            

    <p>!<a href="/url" title="title">foo</a></p>
            

## <a href="#TOC" class="toc-link"></a><span class="number">6.8</span>Autolinks

<a href="#autolink" id="autolink" class="definition">Autolink</a>s are
absolute URIs and email addresses inside `<` and `>`. They are parsed as
links, with the URL or email address as the link label.

A <a href="#uri-autolink" id="uri-autolink" class="definition">URI
autolink</a> consists of `<`, followed by an [absolute
URI](#absolute-uri) followed by `>`. It is parsed as a link to the URI,
with the URI as the link’s label.

An <a href="#absolute-uri" id="absolute-uri" class="definition">absolute
URI</a>, for these purposes, consists of a [scheme](#scheme) followed by
a colon (`:`) followed by zero or more characters other than ASCII
[whitespace](#whitespace) and control characters, `<`, and `>`. If the
URI includes these characters, they must be percent-encoded (e.g. `%20`
for a space).

For purposes of this spec, a
<a href="#scheme" id="scheme" class="definition">scheme</a> is any
sequence of 2–32 characters beginning with an ASCII letter and followed
by any combination of ASCII letters, digits, or the symbols plus (“+”),
period (“.”), or hyphen (“-”).

Here are some valid autolinks:

[Example 603](#example-603)

    <http://foo.bar.baz>
            

    <p><a
              href="http://foo.bar.baz">http://foo.bar.baz</a></p>
            

[Example 604](#example-604)

    <http://foo.bar.baz/test?q=hello&id=22&boolean>
            

    <p><a
              href="http://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean">http://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean</a></p>
            

[Example 605](#example-605)

    <irc://foo.bar:2233/baz>
            

    <p><a
              href="irc://foo.bar:2233/baz">irc://foo.bar:2233/baz</a></p>
            

Uppercase is also fine:

[Example 606](#example-606)

    <MAILTO:FOO@BAR.BAZ>
            

    <p><a
              href="MAILTO:FOO@BAR.BAZ">MAILTO:FOO@BAR.BAZ</a></p>
            

Note that many strings that count as [absolute URIs](#absolute-uri) for
purposes of this spec are not valid URIs, because their schemes are not
registered or because of other problems with their syntax:

[Example 607](#example-607)

    <a+b+c:d>
            

    <p><a
              href="a+b+c:d">a+b+c:d</a></p>
            

[Example 608](#example-608)

    <made-up-scheme://foo,bar>
            

    <p><a
              href="made-up-scheme://foo,bar">made-up-scheme://foo,bar</a></p>
            

[Example 609](#example-609)

    <http://../>
            

    <p><a
              href="http://../">http://../</a></p>
            

[Example 610](#example-610)

    <localhost:5001/foo>
            

    <p><a
              href="localhost:5001/foo">localhost:5001/foo</a></p>
            

Spaces are not allowed in autolinks:

[Example 611](#example-611)

    <http://foo.bar/baz bim>
            

    <p>&lt;http://foo.bar/baz
              bim&gt;</p>
            

Backslash-escapes do not work inside autolinks:

[Example 612](#example-612)

    <http://example.com/\[\>
            

    <p><a
              href="http://example.com/%5C%5B%5C">http://example.com/\[\</a></p>
            

An
<a href="#email-autolink" id="email-autolink" class="definition">email
autolink</a> consists of `<`, followed by an [email
address](#email-address), followed by `>`. The link’s label is the email
address, and the URL is `mailto:` followed by the email address.

An <a href="#email-address" id="email-address" class="definition">email
address</a>, for these purposes, is anything that matches the
[non-normative regex from the HTML5
spec](https://html.spec.whatwg.org/multipage/forms.html#e-mail-state-(type=email)):

    /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?
          (?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/
        

Examples of email autolinks:

[Example 613](#example-613)

    <foo@bar.example.com>
            

    <p><a
              href="mailto:foo@bar.example.com">foo@bar.example.com</a></p>
            

[Example 614](#example-614)

    <foo+special@Bar.baz-bar0.com>
            

    <p><a
              href="mailto:foo+special@Bar.baz-bar0.com">foo+special@Bar.baz-bar0.com</a></p>
            

Backslash-escapes do not work inside email autolinks:

[Example 615](#example-615)

    <foo\+@bar.example.com>
            

    <p>&lt;foo+@bar.example.com&gt;</p>
            

These are not autolinks:

[Example 616](#example-616)

    <>
            

    <p>&lt;&gt;</p>
            

[Example 617](#example-617)

    < http://foo.bar
              >
            

    <p>&lt; http://foo.bar
              &gt;</p>
            

[Example 618](#example-618)

    <m:abc>
            

    <p>&lt;m:abc&gt;</p>
            

[Example 619](#example-619)

    <foo.bar.baz>
            

    <p>&lt;foo.bar.baz&gt;</p>
            

[Example 620](#example-620)

    http://example.com
            

    <p>http://example.com</p>
            

[Example 621](#example-621)

    foo@bar.example.com
            

    <p>foo@bar.example.com</p>
            

## <a href="#TOC" class="toc-link"></a><span class="number">6.9</span>Autolinks (extension)

GFM enables the `autolink` extension, where autolinks will be recognised
in a greater number of conditions.

[Autolink](#autolink)s can also be constructed without requiring the use
of `<` and to `>` to delimit them, although they will be recognized
under a smaller set of circumstances. All such recognized autolinks can
only come at the beginning of a line, after whitespace, or any of the
delimiting characters `*`, `_`, `~`, and `(`.

An <a href="#extended-www-autolink" id="extended-www-autolink"
class="definition">extended www autolink</a> will be recognized when the
text `www.` is found followed by a [valid domain](#valid-domain). A
<a href="#valid-domain" id="valid-domain" class="definition">valid
domain</a> consists of segments of alphanumeric characters, underscores
(`_`) and hyphens (`-`) separated by periods (`.`). There must be at
least one period, and no underscores may be present in the last two
segments of the domain.

The scheme `http` will be inserted automatically:

[Example 622](#example-622)

    www.commonmark.org
              

    <p><a
                href="http://www.commonmark.org">www.commonmark.org</a></p>
              

After a [valid domain](#valid-domain), zero or more non-space non-`<`
characters may follow:

[Example 623](#example-623)

    Visit www.commonmark.org/help for more information.
              

    <p>Visit <a
                href="http://www.commonmark.org/help">www.commonmark.org/help</a> for more
                information.</p>
              

We then apply <a href="#extended-autolink-path-validation"
id="extended-autolink-path-validation" class="definition">extended
autolink path validation</a> as follows:

Trailing punctuation (specifically, `?`, `!`, `.`, `,`, `:`, `*`, `_`,
and `~`) will not be considered part of the autolink, though they may be
included in the interior of the link:

[Example 624](#example-624)

    Visit www.commonmark.org.

                Visit www.commonmark.org/a.b.
              

    <p>Visit <a
                href="http://www.commonmark.org">www.commonmark.org</a>.</p>
                <p>Visit <a
                href="http://www.commonmark.org/a.b">www.commonmark.org/a.b</a>.</p>
              

When an autolink ends in `)`, we scan the entire autolink for the total
number of parentheses. If there is a greater number of closing
parentheses than opening ones, we don’t consider the unmatched trailing
parentheses part of the autolink, in order to facilitate including an
autolink inside a parenthesis:

[Example 625](#example-625)

    www.google.com/search?q=Markup+(business)

                www.google.com/search?q=Markup+(business)))

                (www.google.com/search?q=Markup+(business))

                (www.google.com/search?q=Markup+(business)
              

    <p><a
                href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a></p>
                <p><a
                href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a>))</p>
                <p>(<a
                href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a>)</p>
                <p>(<a
                href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a></p>
              

This check is only done when the link ends in a closing parentheses `)`,
so if the only parentheses are in the interior of the autolink, no
special rules are applied:

[Example 626](#example-626)

    www.google.com/search?q=(business))+ok
              

    <p><a
                href="http://www.google.com/search?q=(business))+ok">www.google.com/search?q=(business))+ok</a></p>
              

If an autolink ends in a semicolon (`;`), we check to see if it appears
to resemble an [entity reference](#entity-references); if the preceding
text is `&` followed by one or more alphanumeric characters. If so, it
is excluded from the autolink:

[Example 627](#example-627)

    www.google.com/search?q=commonmark&hl=en

                www.google.com/search?q=commonmark&hl;
              

    <p><a
                href="http://www.google.com/search?q=commonmark&amp;hl=en">www.google.com/search?q=commonmark&amp;hl=en</a></p>
                <p><a
                href="http://www.google.com/search?q=commonmark">www.google.com/search?q=commonmark</a>&amp;hl;</p>
              

`<` immediately ends an autolink.

[Example 628](#example-628)

    www.commonmark.org/he<lp
              

    <p><a
                href="http://www.commonmark.org/he">www.commonmark.org/he</a>&lt;lp</p>
              

An <a href="#extended-url-autolink" id="extended-url-autolink"
class="definition">extended url autolink</a> will be recognised when one
of the schemes `http://`, or `https://`, followed by a [valid
domain](#valid-domain), then zero or more non-space non-`<` characters
according to [extended autolink path
validation](#extended-autolink-path-validation):

[Example 629](#example-629)

    http://commonmark.org

                (Visit https://encrypted.google.com/search?q=Markup+(business))
              

    <p><a
                href="http://commonmark.org">http://commonmark.org</a></p>
                <p>(Visit <a
                href="https://encrypted.google.com/search?q=Markup+(business)">https://encrypted.google.com/search?q=Markup+(business)</a>)</p>
              

An <a href="#extended-email-autolink" id="extended-email-autolink"
class="definition">extended email autolink</a> will be recognised when
an email address is recognised within any text node. Email addresses are
recognised according to the following rules:

- One ore more characters which are alphanumeric, or `.`, `-`, `_`, or
  `+`.
- An `@` symbol.
- One or more characters which are alphanumeric, or `-` or `_`,
  separated by periods (`.`). There must be at least one period. The
  last character must not be one of `-` or `_`.

The scheme `mailto:` will automatically be added to the generated link:

[Example 630](#example-630)

    foo@bar.baz
              

    <p><a
                href="mailto:foo@bar.baz">foo@bar.baz</a></p>
              

`+` can occur before the `@`, but not after.

[Example 631](#example-631)

    hello@mail+xyz.example isn't
                valid, but hello+xyz@mail.example is.
              

    <p>hello@mail+xyz.example isn't valid, but <a
                href="mailto:hello+xyz@mail.example">hello+xyz@mail.example</a> is.</p>
              

`.`, `-`, and `_` can occur on both sides of the `@`, but only `.` may
occur at the end of the email address, in which case it will not be
considered part of the address:

[Example 632](#example-632)

    a.b-c_d@a.b

                a.b-c_d@a.b.

                a.b-c_d@a.b-

                a.b-c_d@a.b_
              

    <p><a
                href="mailto:a.b-c_d@a.b">a.b-c_d@a.b</a></p>
                <p><a
                href="mailto:a.b-c_d@a.b">a.b-c_d@a.b</a>.</p>
                <p>a.b-c_d@a.b-</p>
                <p>a.b-c_d@a.b_</p>
              

An <a href="#extended-protocol-autolink" id="extended-protocol-autolink"
class="definition">extended protocol autolink</a> will be recognised
when a protocol is recognised within any text node. Valid protocols are:

- `mailto:`
- `xmpp:`

The scheme of the protocol will automatically be added to the generated
link. All the rules of email address autolinking apply.

[Example 633](#example-633)

    mailto:foo@bar.baz

                mailto:a.b-c_d@a.b

                mailto:a.b-c_d@a.b.

                mailto:a.b-c_d@a.b/

                mailto:a.b-c_d@a.b-

                mailto:a.b-c_d@a.b_

                xmpp:foo@bar.baz

                xmpp:foo@bar.baz.
              

    <p><a
                href="mailto:foo@bar.baz">mailto:foo@bar.baz</a></p>
                <p><a
                href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a></p>
                <p><a
                href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a>.</p>
                <p><a
                href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a>/</p>
                <p>mailto:a.b-c_d@a.b-</p>
                <p>mailto:a.b-c_d@a.b_</p>
                <p><a
                href="xmpp:foo@bar.baz">xmpp:foo@bar.baz</a></p>
                <p><a
                href="xmpp:foo@bar.baz">xmpp:foo@bar.baz</a>.</p>
              

A described in the
[specification](https://datatracker.ietf.org/doc/rfc7622/) `xmpp` offers
an optional `/` followed by a resource. The resource can contain all
alphanumeric characters, as well as `@` and `.`.

[Example 634](#example-634)

    xmpp:foo@bar.baz/txt

                xmpp:foo@bar.baz/txt@bin

                xmpp:foo@bar.baz/txt@bin.com
              

    <p><a
                href="xmpp:foo@bar.baz/txt">xmpp:foo@bar.baz/txt</a></p>
                <p><a
                href="xmpp:foo@bar.baz/txt@bin">xmpp:foo@bar.baz/txt@bin</a></p>
                <p><a
                href="xmpp:foo@bar.baz/txt@bin.com">xmpp:foo@bar.baz/txt@bin.com</a></p>
              

Further `/` characters are not considered part of the domain:

[Example 635](#example-635)

    xmpp:foo@bar.baz/txt/bin
              

    <p><a
                href="xmpp:foo@bar.baz/txt">xmpp:foo@bar.baz/txt</a>/bin</p>
              

## <a href="#TOC" class="toc-link"></a><span class="number">6.10</span>Raw HTML

Text between `<` and `>` that looks like an HTML tag is parsed as a raw
HTML tag and will be rendered in HTML without escaping. Tag and
attribute names are not limited to current HTML tags, so custom tags
(and even, say, DocBook tags) may be used.

Here is the grammar for tags:

A <a href="#tag-name" id="tag-name" class="definition">tag name</a>
consists of an ASCII letter followed by zero or more ASCII letters,
digits, or hyphens (`-`).

An <a href="#attribute" id="attribute" class="definition">attribute</a>
consists of [whitespace](#whitespace), an [attribute
name](#attribute-name), and an optional [attribute value
specification](#attribute-value-specification).

An <a href="#attribute-name" id="attribute-name"
class="definition">attribute name</a> consists of an ASCII letter, `_`,
or `:`, followed by zero or more ASCII letters, digits, `_`, `.`, `:`,
or `-`. (Note: This is the XML specification restricted to ASCII. HTML5
is laxer.)

An <a href="#attribute-value-specification"
id="attribute-value-specification" class="definition">attribute value
specification</a> consists of optional [whitespace](#whitespace), a `=`
character, optional [whitespace](#whitespace), and an [attribute
value](#attribute-value).

An <a href="#attribute-value" id="attribute-value"
class="definition">attribute value</a> consists of an [unquoted
attribute value](#unquoted-attribute-value), a [single-quoted attribute
value](#single-quoted-attribute-value), or a [double-quoted attribute
value](#double-quoted-attribute-value).

An <a href="#unquoted-attribute-value" id="unquoted-attribute-value"
class="definition">unquoted attribute value</a> is a nonempty string of
characters not including [whitespace](#whitespace), `"`, `'`, `=`, `<`,
`>`, or `` ` ``.

A <a href="#single-quoted-attribute-value"
id="single-quoted-attribute-value" class="definition">single-quoted
attribute value</a> consists of `'`, zero or more characters not
including `'`, and a final `'`.

A <a href="#double-quoted-attribute-value"
id="double-quoted-attribute-value" class="definition">double-quoted
attribute value</a> consists of `"`, zero or more characters not
including `"`, and a final `"`.

An <a href="#open-tag" id="open-tag" class="definition">open tag</a>
consists of a `<` character, a [tag name](#tag-name), zero or more
[attributes](#attribute), optional [whitespace](#whitespace), an
optional `/` character, and a `>` character.

A <a href="#closing-tag" id="closing-tag" class="definition">closing
tag</a> consists of the string `</`, a [tag name](#tag-name), optional
[whitespace](#whitespace), and the character `>`.

An <a href="#html-comment" id="html-comment" class="definition">HTML
comment</a> consists of `<!--` + *text* + `-->`, where *text* does not
start with `>` or `->`, does not end with `-`, and does not contain
`--`. (See the [HTML5
spec](http://www.w3.org/TR/html5/syntax.html#comments).)

A <a href="#processing-instruction" id="processing-instruction"
class="definition">processing instruction</a> consists of the string
`<?`, a string of characters not including the string `?>`, and the
string `?>`.

A <a href="#declaration" id="declaration"
class="definition">declaration</a> consists of the string `<!`, a name
consisting of one or more uppercase ASCII letters,
[whitespace](#whitespace), a string of characters not including the
character `>`, and the character `>`.

A <a href="#cdata-section" id="cdata-section" class="definition">CDATA
section</a> consists of the string `<![CDATA[`, a string of characters
not including the string `]]>`, and the string `]]>`.

An <a href="#html-tag" id="html-tag" class="definition">HTML tag</a>
consists of an [open tag](#open-tag), a [closing tag](#closing-tag), an
[HTML comment](#html-comment), a [processing
instruction](#processing-instruction), a [declaration](#declaration), or
a [CDATA section](#cdata-section).

Here are some simple open tags:

[Example 636](#example-636)

    <a><bab><c2c>
            

    <p><a><bab><c2c></p>
            

Empty elements:

[Example 637](#example-637)

    <a/><b2/>
            

    <p><a/><b2/></p>
            

[Whitespace](#whitespace) is allowed:

[Example 638](#example-638)

    <a  /><b2
              data="foo" >
            

    <p><a  /><b2
              data="foo" ></p>
            

With attributes:

[Example 639](#example-639)

    <a foo="bar"
              bam = 'baz
              <em>"</em>'
              _boolean zoop:33=zoop:33 />
            

    <p><a foo="bar"
              bam = 'baz
              <em>"</em>'
              _boolean zoop:33=zoop:33 /></p>
            

Custom tag names can be used:

[Example 640](#example-640)

    Foo <responsive-image
              src="foo.jpg" />
            

    <p>Foo <responsive-image
              src="foo.jpg" /></p>
            

Illegal tag names, not parsed as HTML:

[Example 641](#example-641)

    <33> <__>
            

    <p>&lt;33&gt;
              &lt;__&gt;</p>
            

Illegal attribute names:

[Example 642](#example-642)

    <a h*#ref="hi">
            

    <p>&lt;a
              h*#ref=&quot;hi&quot;&gt;</p>
            

Illegal attribute values:

[Example 643](#example-643)

    <a href="hi'>
              <a href=hi'>
            

    <p>&lt;a href=&quot;hi'&gt; &lt;a href=hi'&gt;</p>
            

Illegal [whitespace](#whitespace):

[Example 644](#example-644)

    < a><
              foo><bar/ >
              <foo bar=baz
              bim!bop />
            

    <p>&lt; a&gt;&lt;
              foo&gt;&lt;bar/ &gt;
              &lt;foo bar=baz
              bim!bop /&gt;</p>
            

Missing [whitespace](#whitespace):

[Example 645](#example-645)

    <a href='bar'title=title>
            

    <p>&lt;a
              href='bar'title=title&gt;</p>
            

Closing tags:

[Example 646](#example-646)

    </a></foo >
            

    <p></a></foo ></p>
            

Illegal attributes in closing tag:

[Example 647](#example-647)

    </a href="foo">
            

    <p>&lt;/a
              href=&quot;foo&quot;&gt;</p>
            

Comments:

[Example 648](#example-648)

    foo <!-- this is a
              comment - with hyphen -->
            

    <p>foo <!--
              this is a
              comment - with hyphen --></p>
            

[Example 649](#example-649)

    foo <!-- not a comment --
              two hyphens -->
            

    <p>foo &lt;!--
              not a comment -- two hyphens --&gt;</p>
            

Not comments:

[Example 650](#example-650)

    foo <!-->
              foo -->

              foo <!-- foo--->
            

    <p>foo &lt;!--&gt;
              foo --&gt;</p>
              <p>foo &lt;!-- foo---&gt;</p>
            

Processing instructions:

[Example 651](#example-651)

    foo <?php echo $a; ?>
            

    <p>foo <?php
              echo $a; ?></p>
            

Declarations:

[Example 652](#example-652)

    foo <!ELEMENT
              br EMPTY>
            

    <p>foo <!ELEMENT
              br EMPTY></p>
            

CDATA sections:

[Example 653](#example-653)

    foo <![CDATA[>&<]]>
            

    <p>foo <![CDATA[>&<]]></p>
            

Entity and numeric character references are preserved in HTML
attributes:

[Example 654](#example-654)

    foo <a
              href="&ouml;">
            

    <p>foo <a
              href="&ouml;"></p>
            

Backslash escapes do not work in HTML attributes:

[Example 655](#example-655)

    foo <a
              href="\*">
            

    <p>foo <a
              href="\*"></p>
            

[Example 656](#example-656)

    <a href="\"">
            

    <p>&lt;a
              href=&quot;&quot;&quot;&gt;</p>
            

## <a href="#TOC" class="toc-link"></a><span class="number">6.11</span>Disallowed Raw HTML (extension)

GFM enables the `tagfilter` extension, where the following HTML tags
will be filtered when rendering HTML output:

- `<title>`
- `<textarea>`
- `<style>`
- `<xmp>`
- `<iframe>`
- `<noembed>`
- `<noframes>`
- `<script>`
- `<plaintext>`

Filtering is done by replacing the leading `<` with the entity `&lt;`.
These tags are chosen in particular as they change how HTML is
interpreted in a way unique to them (i.e. nested HTML is interpreted
differently), and this is usually undesireable in the context of other
rendered Markdown content.

All other HTML tags are left untouched.

[Example 657](#example-657)

    <strong> <title>
                <style> <em>

                <blockquote>
                  <xmp> is disallowed.  <XMP> is also disallowed.
                </blockquote>
              

    <p><strong> &lt;title> &lt;style> <em></p>
                <blockquote>
                  &lt;xmp> is disallowed. 
                &lt;XMP> is also
                disallowed.
                </blockquote>
              

## <a href="#TOC" class="toc-link"></a><span class="number">6.12</span>Hard line breaks

A line break (not in a code span or HTML tag) that is preceded by two or
more spaces and does not occur at the end of a block is parsed as a
<a href="#hard-line-break" id="hard-line-break" class="definition">hard
line break</a> (rendered in HTML as a `<br />` tag):

[Example 658](#example-658)

    foo  
              baz
            

    <p>foo<br />
              baz</p>
            

For a more visible alternative, a backslash before the [line
ending](#line-ending) may be used instead of two spaces:

[Example 659](#example-659)

    foo\
              baz
            

    <p>foo<br />
              baz</p>
            

More than two spaces can be used:

[Example 660](#example-660)

    foo     
               
              baz
            

    <p>foo<br />
              baz</p>
            

Leading spaces at the beginning of the next line are ignored:

[Example 661](#example-661)

    foo  
                 
               bar
            

    <p>foo<br />
              bar</p>
            

[Example 662](#example-662)

    foo\
                 
               bar
            

    <p>foo<br />
              bar</p>
            

Line breaks can occur inside emphasis, links, and other constructs that
allow inline content:

[Example 663](#example-663)

    *foo  
              bar*
            

    <p><em>foo<br />
              bar</em></p>
            

[Example 664](#example-664)

    *foo\
              bar*
            

    <p><em>foo<br />
              bar</em></p>
            

Line breaks do not occur inside code spans

[Example 665](#example-665)

    `code  
              span`
            

    <p><code>code 
               span</code></p>
            

[Example 666](#example-666)

    `code\
              span`
            

    <p><code>code\ span</code></p>
            

or HTML tags:

[Example 667](#example-667)

    <a href="foo
               
              bar">
            

    <p><a href="foo
               
              bar"></p>
            

[Example 668](#example-668)

    <a href="foo\
              bar">
            

    <p><a href="foo\
              bar"></p>
            

Hard line breaks are for separating inline content within a block.
Neither syntax for hard line breaks works at the end of a paragraph or
other block element:

[Example 669](#example-669)

    foo\
            

    <p>foo\</p>
            

[Example 670](#example-670)

    foo  
            

    <p>foo</p>
            

[Example 671](#example-671)

    ### foo\
            

    <h3>foo\</h3>
            

[Example 672](#example-672)

    ### foo  
            

    <h3>foo</h3>
            

## <a href="#TOC" class="toc-link"></a><span class="number">6.13</span>Soft line breaks

A regular line break (not in a code span or HTML tag) that is not
preceded by two or more spaces or a backslash is parsed as a
<a href="#softbreak" id="softbreak" class="definition">softbreak</a>. (A
softbreak may be rendered in HTML either as a [line
ending](#line-ending) or as a space. The result will be the same in
browsers. In the examples here, a [line ending](#line-ending) will be
used.)

[Example 673](#example-673)

    foo
              baz
            

    <p>foo
              baz</p>
            

Spaces at the end of the line and beginning of the next line are
removed:

[Example 674](#example-674)

    foo 
               baz
            

    <p>foo
              baz</p>
            

A conforming parser may render a soft line break in HTML either as a
line break or as a space.

A renderer may also provide an option to render soft line breaks as hard
line breaks.

## <a href="#TOC" class="toc-link"></a><span class="number">6.14</span>Textual content

Any characters not given an interpretation by the above rules will be
parsed as plain textual content.

[Example 675](#example-675)

    hello $.;'there
            

    <p>hello $.;'there</p>
            

[Example 676](#example-676)

    Foo χρῆν
            

    <p>Foo χρῆν</p>
            

Internal spaces are preserved verbatim:

[Example 677](#example-677)

    Multiple     spaces
            

    <p>Multiple     spaces</p>
            

# Appendix: A parsing strategy

In this appendix we describe some features of the parsing strategy used
in the CommonMark reference implementations.

## Overview

Parsing has two phases:

1.  In the first phase, lines of input are consumed and the block
    structure of the document—its division into paragraphs, block
    quotes, list items, and so on—is constructed. Text is assigned to
    these blocks but not parsed. Link reference definitions are parsed
    and a map of links is constructed.

2.  In the second phase, the raw text contents of paragraphs and
    headings are parsed into sequences of Markdown inline elements
    (strings, code spans, links, emphasis, and so on), using the map of
    link references constructed in phase 1.

At each point in processing, the document is represented as a tree of
**blocks**. The root of the tree is a `document` block. The `document`
may have any number of other blocks as **children**. These children may,
in turn, have other blocks as children. The last child of a block is
normally considered **open**, meaning that subsequent lines of input can
alter its contents. (Blocks that are not open are **closed**.) Here, for
example, is a possible document tree, with the open blocks marked by
arrows:

    -> document
          -> block_quote
          paragraph
          "Lorem ipsum dolor\nsit amet."
          -> list (type=bullet tight=true bullet_char=-)
          list_item
          paragraph
          "Qui *quodsi iracundia*"
          -> list_item
          -> paragraph
          "aliquando id"
        

## Phase 1: block structure

Each line that is processed has an effect on this tree. The line is
analyzed and, depending on its contents, the document may be altered in
one or more of the following ways:

1.  One or more open blocks may be closed.
2.  One or more new blocks may be created as children of the last open
    block.
3.  Text may be added to the last (deepest) open block remaining on the
    tree.

Once a line has been incorporated into the tree in this way, it can be
discarded, so input can be read in a stream.

For each line, we follow this procedure:

1.  First we iterate through the open blocks, starting with the root
    document, and descending through last children down to the last open
    block. Each block imposes a condition that the line must satisfy if
    the block is to remain open. For example, a block quote requires a
    `>` character. A paragraph requires a non-blank line. In this phase
    we may match all or just some of the open blocks. But we cannot
    close unmatched blocks yet, because we may have a [lazy continuation
    line](#lazy-continuation-line).

2.  Next, after consuming the continuation markers for existing blocks,
    we look for new block starts (e.g. `>` for a block quote). If we
    encounter a new block start, we close any blocks unmatched in step 1
    before creating the new block as a child of the last matched block.

3.  Finally, we look at the remainder of the line (after block markers
    like `>`, list markers, and indentation have been consumed). This is
    text that can be incorporated into the last open block (a paragraph,
    code block, heading, or raw HTML).

Setext headings are formed when we see a line of a paragraph that is a
[setext heading underline](#setext-heading-underline).

Reference link definitions are detected when a paragraph is closed; the
accumulated text lines are parsed to see if they begin with one or more
reference link definitions. Any remainder becomes a normal paragraph.

We can see how this works by considering how the tree above is generated
by four lines of Markdown:

    > Lorem ipsum dolor
          sit amet.
          > - Qui *quodsi iracundia*
          > - aliquando id
        

At the outset, our document model is just

    -> document
        

The first line of our text,

    > Lorem ipsum dolor
        

causes a `block_quote` block to be created as a child of our open
`document` block, and a `paragraph` block as a child of the
`block_quote`. Then the text is added to the last open block, the
`paragraph`:

    -> document
          -> block_quote
          -> paragraph
          "Lorem ipsum dolor"
        

The next line,

    sit amet.
        

is a “lazy continuation” of the open `paragraph`, so it gets added to
the paragraph’s text:

    -> document
          -> block_quote
          -> paragraph
          "Lorem ipsum dolor\nsit amet."
        

The third line,

    > - Qui *quodsi iracundia*
        

causes the `paragraph` block to be closed, and a new `list` block opened
as a child of the `block_quote`. A `list_item` is also added as a child
of the `list`, and a `paragraph` as a child of the `list_item`. The text
is then added to the new `paragraph`:

    -> document
          -> block_quote
          paragraph
          "Lorem ipsum dolor\nsit amet."
          -> list (type=bullet tight=true bullet_char=-)
          -> list_item
          -> paragraph
          "Qui *quodsi iracundia*"
        

The fourth line,

    > - aliquando id
        

causes the `list_item` (and its child the `paragraph`) to be closed, and
a new `list_item` opened up as child of the `list`. A `paragraph` is
added as a child of the new `list_item`, to contain the text. We thus
obtain the final tree:

    -> document
          -> block_quote
          paragraph
          "Lorem ipsum dolor\nsit amet."
          -> list (type=bullet tight=true bullet_char=-)
          list_item
          paragraph
          "Qui *quodsi iracundia*"
          -> list_item
          -> paragraph
          "aliquando id"
        

## Phase 2: inline structure

Once all of the input has been parsed, all open blocks are closed.

We then “walk the tree,” visiting every node, and parse raw string
contents of paragraphs and headings as inlines. At this point we have
seen all the link reference definitions, so we can resolve reference
links as we go.

    document
          block_quote
          paragraph
          str "Lorem ipsum dolor"
          softbreak
          str "sit amet."
          list (type=bullet tight=true bullet_char=-)
          list_item
          paragraph
          str "Qui "
          emph
          str "quodsi iracundia"
          list_item
          paragraph
          str "aliquando id"
        

Notice how the [line ending](#line-ending) in the first paragraph has
been parsed as a `softbreak`, and the asterisks in the first list item
have become an `emph`.

### An algorithm for parsing nested emphasis and links

By far the trickiest part of inline parsing is handling emphasis, strong
emphasis, links, and images. This is done using the following algorithm.

When we’re parsing inlines and we hit either

- a run of `*` or `_` characters, or
- a `[` or `![`

we insert a text node with these symbols as its literal content, and we
add a pointer to this text node to the
<a href="#delimiter-stack" id="delimiter-stack"
class="definition">delimiter stack</a>.

The [delimiter stack](#delimiter-stack) is a doubly linked list. Each
element contains a pointer to a text node, plus information about

- the type of delimiter (`[`, `![`, `*`, `_`)
- the number of delimiters,
- whether the delimiter is “active” (all are active to start), and
- whether the delimiter is a potential opener, a potential closer, or
  both (which depends on what sort of characters precede and follow the
  delimiters).

When we hit a `]` character, we call the *look for link or image*
procedure (see below).

When we hit the end of the input, we call the *process emphasis*
procedure (see below), with `stack_bottom` = NULL.

#### *look for link or image*

Starting at the top of the delimiter stack, we look backwards through
the stack for an opening `[` or `![` delimiter.

- If we don’t find one, we return a literal text node `]`.

- If we do find one, but it’s not *active*, we remove the inactive
  delimiter from the stack, and return a literal text node `]`.

- If we find one and it’s active, then we parse ahead to see if we have
  an inline link/image, reference link/image, compact reference
  link/image, or shortcut reference link/image.

  - If we don’t, then we remove the opening delimiter from the delimiter
    stack and return a literal text node `]`.

  - If we do, then

    - We return a link or image node whose children are the inlines
      after the text node pointed to by the opening delimiter.

    - We run *process emphasis* on these inlines, with the `[` opener as
      `stack_bottom`.

    - We remove the opening delimiter.

    - If we have a link (and not an image), we also set all `[`
      delimiters before the opening delimiter to *inactive*. (This will
      prevent us from getting links within links.)

#### *process emphasis*

Parameter `stack_bottom` sets a lower bound to how far we descend in the
[delimiter stack](#delimiter-stack). If it is NULL, we can go all the
way to the bottom. Otherwise, we stop before visiting `stack_bottom`.

Let `current_position` point to the element on the [delimiter
stack](#delimiter-stack) just above `stack_bottom` (or the first element
if `stack_bottom` is NULL).

We keep track of the `openers_bottom` for each delimiter type (`*`, `_`)
and each length of the closing delimiter run (modulo 3). Initialize this
to `stack_bottom`.

Then we repeat the following until we run out of potential closers:

- Move `current_position` forward in the delimiter stack (if needed)
  until we find the first potential closer with delimiter `*` or `_`.
  (This will be the potential closer closest to the beginning of the
  input – the first one in parse order.)

- Now, look back in the stack (staying above `stack_bottom` and the
  `openers_bottom` for this delimiter type) for the first matching
  potential opener (“matching” means same delimiter).

- If one is found:

  - Figure out whether we have emphasis or strong emphasis: if both
    closer and opener spans have length &gt;= 2, we have strong,
    otherwise regular.

  - Insert an emph or strong emph node accordingly, after the text node
    corresponding to the opener.

  - Remove any delimiters between the opener and closer from the
    delimiter stack.

  - Remove 1 (for regular emph) or 2 (for strong emph) delimiters from
    the opening and closing text nodes. If they become empty as a
    result, remove them and remove the corresponding element of the
    delimiter stack. If the closing node is removed, reset
    `current_position` to the next element in the stack.

- If none is found:

  - Set `openers_bottom` to the element before `current_position`. (We
    know that there are no openers for this kind of closer up to and
    including this point, so this puts a lower bound on future
    searches.)

  - If the closer at `current_position` is not a potential opener,
    remove it from the delimiter stack (since we know it can’t be a
    closer either).

  - Advance `current_position` to the next element in the stack.

After we’re done, we remove all delimiters above `stack_bottom` from the
delimiter stack.
