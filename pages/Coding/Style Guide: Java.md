# Texas Torque Java Style Guide

By [Justus Languell](https://www.justusl.com/), loosely based of the [Google Java Sytle Guide](https://google.github.io/styleguide/javaguide.html), with a lot of personal opinion.

## Contents

* [File Struture](#File-Struture)
* [Formatting](#Formatting)
* [Naming](#Naming)
* [Programming Practices](#Programming-Practices)
* [Javadoc](#Javadoc)

## File Struture

### File name

The source file name consists of the case-sensitive name of the top-level class it contains (of which there is exactly one), plus the `.java` extension.

### Characters

Files are encoded in UTF-8.

Indentation is to be done with **spaces only**.

For any character that has a special escape sequence (`\b`, `\t`, `\n`, `\f`, `\r`, `\"`, `\'` and `\\`), that sequence is used rather than the corresponding octal (e.g. `\012`) or Unicode (e.g. `\u000a`) escape.

For the remaining non-ASCII characters, either the actual Unicode character (e.g. `∞`) or the equivalent Unicode escape (e.g. `\u221e`) is used. The Unicode character is perfered, but th escape is to be used if there is no suitable character, and must be accompanied with a comment.

### Source Structure

A source file must be structured as follows:

* License or package information.
* Package statement (not line-wrapped).
* Import statements (not line-wrapped).
* Exactly one top-level class.

**Exactly one** blank lines between the sections.

### Import Statements

* Import statements are not line-wrapped.
* Do not use wildcard imports on classes.
* Do not use static imports on classes.
* Put static imports before plain imports.

### Class Layout

Layout beyond what specified below is left to the author.

* Singleton instance if applicable.
* `public static final` constants.
* Class variable fields.
* Constructors
* Methods

## Formatting

You don't need to worry about too much formatting because we use Clang-Format. The Texas Torque Clang-Format for Java is specified as below:

```yaml
Language: Java     
AlignConsecutiveAssignments: false
AlignConsecutiveDeclarations: false
ColumnLimit: 120
ContinuationIndentWidth: 8    
AllowShortBlocksOnASingleLine: true    
AllowShortCaseLabelsOnASingleLine: true    
AllowShortEnumsOnASingleLine: true    
AllowShortFunctionsOnASingleLine: true    
AllowShortIfStatementsOnASingleLine: AllIfsAndElse    
AllowShortLoopsOnASingleLine: true    
AllowShortIfStatementsOnASingleLine: true    
IndentRequires: true
BreakBeforeBraces: Attach
UseTab: Never
BreakAfterJavaFieldAnnotations: true
AlignOperands: true
AlignTrailingComments: true 
```

### Quick rules

* Braces **always go on the same line as the statement**.
* If braces are optional, you don't have to use them, just be consistent with that block of code.
* One line `methods`/`if`/`else`/`for`/`while` are allowed.
* The line limit is 120 characters.
* Indentation is 4 spaces.
* A continuation of the previous line is indented 8 spaces.
* Always break a line on an opeator, start the next line with the operator.
* Horizontal alignment is never required but is optional.
* Single line variable declarations with commas are allowed (ex: `int a, b = 5;`)
* Don't use `switch` statements.
* Grouping paranthesis are recomended.

### The order of field modifiers

You do need to worry about the order of field modifiers (from left to right):

* `private`, `protected`, or `public`
* `static`
* `final`
* `volatile` or `synchronized`

### Horizontal white space

This is ussually taken care of by Clang-Format, but we pretty much use Google's rules.

Beyond where required by the language or other style rules, and apart from literals, comments and Javadoc, a single ASCII space also appears in the following places only.

* Separating any reserved word, such as `if`, `for`, or `catch`, from an open parenthesis (() that follows it on that line
* Separating any reserved word, such as `else` or `catch`, from a closing curly brace (}) that precedes it on that line
* Before any open curly brace (`{`), with two exceptions:
  * `@SomeAnnotation({a, b})` (no space is used)
  * `String[][] x = {{"foo"}};` (no space is required between `{{`, by item 9 below)
* On both sides of any binary or ternary operator. This also applies to the following "operator-like" symbols:
  * the ampersand in a conjunctive type bound: `<T extends Foo & Bar>`
  * the pipe for a catch block that handles multiple exceptions: `catch (FooException | BarException e)`
  * the colon (`:`) in an enhanced `for` ("foreach") statement
the arrow in a lambda expression: `(String str) -> str.length()`
* but not
  * the two colons (`::`) of a method reference, which is written like `Object::toString`
  * the dot separator (`.`), which is written like `object.toString()`
* After `,`, `:`, `;`, or the closing parenthesis (`)`) of a cast
* Between any content and a double slash (`//`) which begins a comment. Multiple spaces are allowed.
* Between a double slash (`//`) which begins a comment and the comment's text. Multiple spaces are allowed.
* Between the type and variable of a declaration: `List<String> list`
* Optional just inside both braces of an array initializer
  * `new int[] {5, 6}` and `new int[] { 5, 6 }` are both valid
* Between a type annotation and `[]` or `...`.

This rule is never interpreted as requiring or forbidding additional space at the start or end of a line; it addresses only interior space.

### Annotations

We use:

* `@Override` (Always use this!)
* `@Deprecated`
* `@FunctionalInterface`
* `@SuppressWarnings`

### Comments

This section addresses implementation comments. Javadoc is addressed separately in Section 7, Javadoc.

Any line break may be preceded by arbitrary whitespace followed by an implementation comment. Such a comment renders the line non-blank.

4.8.6.1 Block comment style
Block comments are indented at the same level as the surrounding code. They may be in `/* ... */` style or `// ...` style. For multi-line `/* ... */` comments, subsequent lines must start with `* aligned with the *` on the previous line.

```java
/*
 * This is          // And so           /* Or you can
 * okay.            // is this.          * even do this. */
 */
```

Comments are not enclosed in boxes drawn with asterisks or other characters.

## Naming

Identifiers use only ASCII letters and digits, and, in a small number of cases noted below, underscores. Thus each valid identifier name is matched by the regular expression \w+ .

Special prefixes or suffixes are not used. No Hungarian case.

Invalid names:

* `name_`
* `mName`
* `s_name`
* `kName`

WPILib loves to use the `mInstanceVariable` and `kConstantOrEnum` bullshit, we do not.

Package names
Package names use only lowercase letters and digits (no underscores). Consecutive words are simply concatenated together. For example, com.example.deepspace, not com.example.deepSpace or com.example.deep_space.

### Class names

Class names are written in UpperCamelCase.

Class names are typically nouns or noun phrases. For example, `Character` or `ImmutableList`. Interface names may also be nouns or noun phrases (ex: `List`), but may sometimes be adjectives or adjective phrases instead (ex: `Readable`).

There are no specific rules or even well-established conventions for naming annotation types.

### Method names

Method names are written in lowerCamelCase.

Method names are typically verbs or verb phrases. For example, `sendMessage` or `stop`.

### Constant names

Constant names use UPPER_SNAKE_CASE: all uppercase letters, with each word separated from the next by a single underscore. But what is a constant, exactly? It's really not that deep, but if you care, read it from [Google](https://google.github.io/styleguide/javaguide.html#s5.2.4-constant-names).

These names are typically nouns or noun phrases.

### Non-constant field names

Non-constant field names (static or otherwise) are written in lowerCamelCase.

These names are typically nouns or noun phrases. For example, `computedValues` or `index`.

### Parameter names

Parameter names are written in lowerCamelCase.

One-character parameter names in public methods should be avoided.

### Local variable names

Local variable names are written in lowerCamelCase.

Even when final and immutable, local variables are not considered to be constants, and should not be styled as constants.

### Type variable names

Each type variable is named in one of two styles:

A single capital letter, optionally followed by a single numeral (such as `T`, `E`, `X`, `T2`).
A name in the form used for classes (see Section 5.2.2, Class names), followed by the capital letter `T` (examples: `RequestT`, `FooBarT`).

## Programming Practices

### `@Override`: always used

A method is marked with the @Override annotation whenever it is legal. This includes a class method overriding a superclass method, a class method implementing an interface method, and an interface method respecifying a superinterface method.

Exception: `@Override` may be omitted when the parent method is `@Deprecated`.

### Static members: qualified using class

When a reference to a static class member must be qualified, it is qualified with that class's name, not with a reference or expression of that class's type.

```java
Foo aFoo = ...;
Foo.aStaticMethod(); // good
aFoo.aStaticMethod(); // bad
somethingThatYieldsAFoo().aStaticMethod(); // very bad
```

### The use of `final`: liberal

We use `final` in every possible context.

* To be overly explicit for readability  purposes
* To prevent unwanted mutation

#### Why?

This mostly applies to fields and local variables. Declaring immutability is important.
It lets us know how a value will change over runtime, and how we can avoid side effects.
This is the same reason I use `final` in parameters declarations. If a parameter is *not*
marked `final` in my code it *will* be mutated. To keep this connotation, we must use `final`
everywhere else. I also use `final` when I can to explicitly specify extendibility. This is
important in building a library, like [TorqueLib](https://github.com/TexasTorque/TorqueLib),
and as a result of this, I use it in the entire Texas Torque codebase. The most extraneous
uses is marking methods that are members of a `final` class or marking static methods as `final`.
The reason I do this is just to maintain consistency with all function declarations. This
is the only case in which I think I may overuse `final`, but it's a habit.

### Finalizers: not used

Don't do it.

## Javadoc

### Javadoc Formatting

The basic formatting of Javadoc blocks is as seen in this example:

```java
/**
 * Multiple lines of Javadoc text are written here,
 * wrapped normally...
 */
public final int method(final String p1) { ... }
```

... or in this single-line example:

```java
/** An especially short bit of Javadoc. */
```

The basic form is always acceptable. The single-line form may be substituted when the entirety of the Javadoc block (including comment markers) can fit on a single line. Note that this only applies when there are no block tags such as `@return`.

### Block tags

Any of the standard "block tags" that are used appear in the order `@param`, `@return`, `@throws`, `@deprecated`, and these four types never appear with an empty description. When a block tag doesn't fit on a single line, continuation lines are indented four (or more) spaces from the position of the `@`.

### Where Javadoc is used

At the minimum, Javadoc is present for every public class, and every public or protected member of such a class, with a few exceptions:

* Javadoc is optional for "simple, obvious" members like `getFoo()`, in cases where there really and truly is nothing else worthwhile to say but "Returns the foo".
* Javadoc is not always present on a method that overrides a supertype method.

