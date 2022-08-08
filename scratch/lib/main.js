import createElement from "./createElement.js";

let count = 0;
const label = createElement(["p"]);
const button = createElement(["button", "Click me"]);
button.addEventListener("click", function (evt) {
  if (count === 0) {
    document.body.append(label);
  }
  label.innerText = `Count: ${++count}`;
});

const hi = createElement(["h1", "Hello, world."]);

document.body.append(hi, button);
