function calculate() {
    const num1 = document.getElementById('num1').value;
    const num2 = document.getElementById('num2').value;
    const operator = document.getElementById('operator').value;

    fetch('http://localhost:8000/arithmetic', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ num1: parseFloat(num1), num2: parseFloat(num2), operator: operator }),
    })
    .then(response => response.json())
    .then(data => {
        document.getElementById('result').innerText = `Result: ${data}`;
    })
    .catch((error) => {
        console.error('Error:', error);
    });
}
