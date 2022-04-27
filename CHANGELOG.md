## v0.5.1 (2022-04-27)

### Fix

- Fixed that the selector doesn't work for multiple class element by @lomirus in #4;

## v0.5.0 (2022-04-16)

### Features

- Add `try_parse` method for the html with tolerable errors by @lomirus in #2;
- Add support for parsing xml doctype by @lomirus in #3;
- Add associated `Doctype` variant to `Node::Doctype` to distinguish between html and xml by @lomirus. 

### Refactor

- Remove `prelude` module, and replace it with `operation` module by @lomirus, which also moves the `Selector` struct out of the original module to top of this package.

### Chore

- Add benchmarking for the `parse` method by @lomirus, using an 87.0kB html file from wikipedia;

### Documentation

- Readme and other documentation improvements by @lomirus.

## v0.4.0 (2022-03-19)

### Fix

- Fails to parse if closing tag is seperated by new-line, like`<a></a\n>`. Closes #1.

### Refactor

- Add the `prelude` module. Now you can simply import `Editable`, `Htmlifiable` and `Queryable` all just by `use html_editor::prelude::*;`;
- `trim` method becomes `(&mut self) -> &mut Self` from `(self) -> Self`.
- Replace the `try_into_element()` with `into_element()`, which simplifies the common use: `try_into_element().unwrap()`

### Documentation

- Add more examples.

## v0.3.0 (2022-01-28)

### Refactor

- Now `parse()` returns a `Result<Vec<Node>, String>` instead of `Vec<Node>` for better error handling.

### Fix

- In previous version, `parse` sometimes may return an unexpected result but without any error message. But now any currently known error will be delivered.
- Tag string in `<script>` or `<style>` cannot be parsed correctly. For example, `<script>"<div></div>"</script>` would be parsed as a `<div>` element in the `<script>` instead of the plain string.

## v0.2.0 (2022-01-19)

### Enhancements

- Omit the attribute if its value is empty. For example:
  ```html
  <!--Old-->
  <script src="index.js" type="module" defer=""></script>

  <!--New-->
  <script src="index.js" type="module" defer></script>
  ```

- Use vector to store the attributes instead of hashmap, which can make its order stable. For example:
  ```html
  <!--Old. When an element turns to html, it may be-->
  <script src="index.js" type="module"></script>
  <!--But below is also a possible result-->
  <script type="module" src="index.js"></script>

  <!--New. The result would be unique, and just the same as its input-->
  <script src="index.js" type="module"></script>
  ```

### Fix

- Fail to parse the attributes if the the last key of it doesn't have the value, like `<script src="index.js" type="module" defer></script>`

## v0.1.0 (2022-01-01)
