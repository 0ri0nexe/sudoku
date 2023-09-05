import { invoke } from "@tauri-apps/api/tauri";

let selectedSquareCoords = [-1, -1]
let sudoku: number[] = [];
for (let i = 0; i < 81; i++) {
  sudoku[i] = 0
}

function setValue(x: number, y: number, value: number) {
  let coord = x + y * 9;
  sudoku[coord] = value;
}

function getSelectedSquare() {
  if (!(selectedSquareCoords[0] >= 0 && selectedSquareCoords[1] >= 0)) {
    return null
  }
  return document.getElementById(`${selectedSquareCoords[0]} ${selectedSquareCoords[1]}`) as HTMLElement
}

function erase() {
  while (document.body.firstChild) {
    document.body.removeChild(document.body.firstChild);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  window.onkeydown = (e) => onKeyPressed(e.key);
  drawGrid(sudoku);
});
function onKeyPressed(key: String) {
  switch (key) {
    case "d":
      solveSudoku();
      break;

    case "n":
      newSudoku();
      break;

    case "1":
    case "2":
    case "3":
    case "4":
    case "5":
    case "6":
    case "7":
    case "8":
    case "9":
      handleNumber(key);
      break;

    case "Backspace":
    case "Delete":
      handleNumber("0")
      break;
    default:
      break;
  }
}

function onSquareClick(x: number, y: number) {

  let selectedSquare = getSelectedSquare();

  if (selectedSquare != null) {
    selectedSquare.style.border = "1px solid #37568D";
    checkBorder((selectedSquare as HTMLDivElement), selectedSquareCoords[0], selectedSquareCoords[1]);
  }
  selectedSquareCoords = [x, y];
  (document.getElementById(`${x} ${y}`) as HTMLElement).style.border = "1px solid red";
}

function drawGrid(sudoku: number[]) {
  for (let i = 0; i < 9; i++) {
    const ligne = document.createElement('div');
    ligne.className = 'ligne';

    for (let j = 0; j < 9; j++) {
      const square = document.createElement('div');
      checkBorder(square, j, i)
      square.className = 'square';
      square.onclick = () => onSquareClick(j, i);
      square.id = `${j} ${i}`

      const number = document.createElement("p");
      let fill_number = sudoku[i * 9 + j]
      number.textContent = fill_number == 0 ? "" : fill_number.toString();
      number.id = "number";

      square.appendChild(number);
      ligne.appendChild(square);
    }

    document.body.appendChild(ligne);
  }
}

function checkBorder(square: HTMLDivElement, x: number, y: number) {
  let boldLikeBorderSettings = "1px solid #000000"
  if (x == 0) {
    square.style.borderLeft = boldLikeBorderSettings;
  }
  if ((x + 1) % 3 == 0) {
    square.style.borderRight = boldLikeBorderSettings;
  }
  if (y == 0) {
    square.style.borderTop = boldLikeBorderSettings;
  }
  if ((y + 1) % 3 == 0) {
    square.style.borderBottom = boldLikeBorderSettings;
  }
}

function handleNumber(str_number: String) {
  let number = parseInt(str_number.toString())
  let selectedSquare = getSelectedSquare()
  if (selectedSquare == null)
    return

  // replace selected square by the number
  let numberElement = selectedSquare.children[0]
  numberElement.textContent = number == 0 ? "" : str_number.toString();
  // replace in the board
  setValue(selectedSquareCoords[0], selectedSquareCoords[1], number);
}

type Payload = {
  sudoku: number[];
}

async function solveSudoku() {
  let to_send: Payload = {
    sudoku: sudoku
  }
  let solvedSudoku: number[];
  let response: Promise<Payload> = invoke("solve_sudoku",
    {
      receivedSudoku: to_send,
    },
  );
  solvedSudoku = (await response).sudoku;
  erase();
  drawGrid(solvedSudoku);
}

async function newSudoku() {
  let response: Promise<Payload> = invoke("get_new_sudoku");
  sudoku = (await response).sudoku;
  erase();
  drawGrid(sudoku);
}