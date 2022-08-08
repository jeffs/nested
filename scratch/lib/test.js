import { AssertionFailure } from "./assert.js";
import testCreateElement from "./testCreateElement.js";

const style = document.createElement("style");
style.append(`
  body {
    background-color: #333;
  }
  table {
    border-collapse: collapse;
    width: 80%;
  }
  td, th {
    border: 1px solid black;
    padding: 4px;
  }
  td {
    background-color: lightgray;
  }
  th {
    background-color: gray;
  }
  tr.fail * {
    background-color: darkred;
    color: lightgray;
  }
`);

document.head.append(style);

const table = document.createElement("table");
const header = document.createElement("tr");
header.append(
  ...["Name", "Status", "Reason"].map((column) => {
    const th = document.createElement("th");
    th.append(column);
    return th;
  })
);
table.append(header);
document.body.append(table);

Object.entries({
  testCreateElement,
}).forEach(([name, test]) => {
  const row = document.createElement("tr");
  const nameCell = document.createElement("td");
  nameCell.append(name);
  const statusCell = document.createElement("td");
  const reasonCell = document.createElement("td");
  row.append(nameCell, statusCell, reasonCell);
  table.append(row);
  try {
    test();
    statusCell.append("PASS");
  } catch (err) {
    row.classList.add("fail");
    statusCell.append("FAIL");
    if (err instanceof AssertionFailure) {
      reasonCell.append(`got ${err.got}, want ${err.want}`);
    }
  }
});
