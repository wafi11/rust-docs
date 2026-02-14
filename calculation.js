function calculation(calculationType, price, amount, percentage) {
    let margin;
    let total;
    switch (calculationType) {
        case "HYBRID":
            margin = Math.round(price * percentage / 100);
            total = price + margin + amount;
            return total;
        case "PERCENTAGE":
            margin = Math.round(price * percentage / 100);
            return margin + price;
        case "FIXED":
            total = amount + price;
            return total;
    }
}

// Benchmark
const iterations = 1_000_000;

console.time("JavaScript Benchmark");
for (let i = 0; i < iterations; i++) {
    calculation("PERCENTAGE", 1000, 0, 0.7);
}
console.timeEnd("JavaScript Benchmark");

// Single execution
const start = performance.now();
const result = calculation("PERCENTAGE", 1000, 0, 0.7);
const end = performance.now();

console.log("Result:", result);
console.log("Single execution:", (end - start).toFixed(6), "ms");