import ItemList from "./ItemList.js";
import createElement from "./createElement.js";

const label = createElement(["p"]);
const button = createElement(["button", "Add item"]);
const list = ItemList();
button.addEventListener("click", function (evt) {
  list.grow();
});

document.body.append(button, list.node());

// XXX
for (let i = 0; i < 10; ++i) {
  list.grow();
}
