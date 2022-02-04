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
