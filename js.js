function* range(n) {
  for (let i = 0; i < n; i++) yield i;
}

let containers = [
  [
    [1, 2],
    [2, 2],
    [3, 1],
  ],
  [
    [4, 1],
    [5, 2],
    [6, 4],
  ],
  [
    [7, 2],
    [8, 2],
    [9, 1],
  ],
];

for (let container_index of range(containers.length)) {
  let container = containers[container_index];
  while (container.length > 0) {
    const el = container.shift();
    containers[(container_index + el[1]) % containers.length].push(el);
  }
}

console.log(
  containers
    .map((container) =>
      container.map((tuple) => `(${tuple[0]}, ${tuple[1]})`).join(" ")
    )
    .map((row, i) => `${i} => ${row}`)
    .join("\n")
);
