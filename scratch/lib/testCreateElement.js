import { assertEq } from "./assert.js";
import createElement from "./createElement.js";

export default function testCreateElement() {
  const style = { color: "blue", listStyleType: "circle" };

  // No style or children.
  let elem = createElement(["ul"]);
  assertEq(elem.tagName, "UL");

  // Children, but no style.
  elem = createElement([
    "ul",
    ["li", ["em", "nested child"]],
    ["li", "second child"],
  ]);
  assertEq(elem.tagName, "UL");
  assertEq(elem.children.length, 2);
  assertEq(elem.firstChild.tagName, "LI");
  assertEq(elem.firstChild.firstChild.tagName, "EM");
  assertEq(elem.firstChild.firstChild.innerText, "nested child");

  // Style, but no children.
  elem = createElement(["ul", style]);
  assertEq(elem.tagName, "UL");
  assertEq(elem.style.color, "blue");

  // Style and children.
  elem = createElement([
    "ul",
    style,
    ["li", ["em", { color: "red" }, "nested child"]],
    ["li", "second child"],
  ]);
  assertEq(elem.tagName, "UL");
  assertEq(elem.style.color, "blue");
  assertEq(elem.children.length, 2);
  assertEq(elem.firstChild.tagName, "LI");
  assertEq(elem.firstChild.firstChild.tagName, "EM");
  assertEq(elem.firstChild.firstChild.style.color, "red");
  assertEq(elem.firstChild.firstChild.innerText, "nested child");
}
