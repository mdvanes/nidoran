import { solve } from "nidoran";

// const result = solve([1,2,3]);
const result = solve([[0, 0, 1, 0, 0, 4, 0, 0, 2], [0, 5, 0, 0, 0, 3, 0, 1, 9], [4, 7, 0, 0, 0, 0, 0, 0, 5], [0, 0, 0, 0, 8, 0, 2, 0, 7], [0, 0, 4, 0, 9, 0, 8, 0, 0], [8, 0, 6, 0, 3, 0, 0, 0, 0], [2, 0, 0, 0, 0, 0, 0, 6, 8], [1, 8, 0, 2, 0, 0, 0, 4, 0], [5, 0, 0, 3, 0, 0, 9, 0, 0]]);

document.getElementById("result").textContent = result;
