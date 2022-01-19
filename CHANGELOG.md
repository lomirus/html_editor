## v0.2.0 (2022-01-19)

## Fix

- Fail to parse the attributes if the the last key of it doesn't have the value, like `<script src="index.js" type="module" defer></script>`

## Enhancements

- Omit the attribute if its value is empty. For example:
  ```html
  <!--Old-->
  <script src="index.js" type="module" defer=""></script>

  <!--New-->
  <script src="index.js" type="module" defer></script>
  ```

- Use vector to store the attributes instead of hashmap, which can make its order stable. For example:
  ```html
  <!--Old. Maybe-->
  <script src="index.js" type="module"></script>
  <!--But it is also a possible result-->
  <script type="module" src="index.js"></script>

  <!--New. The result would be unique, and just the same as its input-->
  <script src="index.js" type="module"></script>
  ```

## v0.1.0 (2022-01-01)
