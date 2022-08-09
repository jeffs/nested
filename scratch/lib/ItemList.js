import createElement from "./createElement.js";

export default function ItemList() {
  let count = 0;
  const node = createElement(["table", { marginTop: "8px" }]);
  return {
    grow() {
      const button = createElement(["button", "x"]);
      const row = createElement([
        "tr",
        ["td", { padding: "8px 4px 8px 8px" }, button],
        ["td", { padding: "8px 8px 8px 4px", textAlign: "right" }, ++count],
      ]);
      button.addEventListener("click", () => {
        row.classList.add("removing");
        //row.remove();
      });
      node.append(row);
    },
    isEmpty() {
      return count === 0;
    },
    node() {
      return node;
    },
  };
}
