// Prototype of raw objects; as opposed to DOM elements, arrays, or other
// Object subclasses.
const rawObjectPrototype = Object.getPrototypeOf({});

// Returns a new DOM element having the specified tag name, optional styles,
// and children.  For example:
//
//  createElement("p", { color: "blue" }, "Hello") => paragraph with blue text
//  createElement("p", "Hello") => paragraph with default text color
//
// Styles and children may be combined, and arrays are processed recursively:
//
//  createElement(
//    "ul",
//    { color: "blue", listStyleType: "circle" },
//    ["li", "first child"],
//    ["li", "second child"],
//  );
export default function createElement([tagName, ...rest]) {
  const elem = document.createElement(tagName);
  if (rest.length) {
    const first = rest[0];
    if (Object.getPrototypeOf(first) === rawObjectPrototype) {
      Object.assign(elem.style, first);
    } else if (Array.isArray(first)) {
      elem.append(createElement(first));
    } else {
      elem.append(first);
    }
    elem.append(
      ...rest
        .slice(1)
        .map((child) => (Array.isArray(child) ? createElement(child) : child))
    );
  }
  return elem;
}
