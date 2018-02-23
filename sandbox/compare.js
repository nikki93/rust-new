console.log('starting');

const startTime = Date.now();

const MAX = 3000000;

let sieve = Array(MAX).fill(true);
for (let i = 2; i < MAX; ++i) {
  if (sieve[i]) {
    let j = i * i;
    while (j < MAX) {
      sieve[j] = false;
      j += i;
    }
  }
}

console.log(sieve[2000107]);

const endTime = Date.now();
console.log((0.001 * (endTime - startTime)).toFixed(3));
