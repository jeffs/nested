import createElement from './createElement.js';

const ul = createElement([
  "ul",
  { color: "blue", listStyleType: "circle" },
  ["li", "first child"],
  ["li", "second child"],
]);

document.body.append(ul);
