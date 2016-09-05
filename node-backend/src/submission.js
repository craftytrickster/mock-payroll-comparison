'use strict';

function processPayment(payment) {
    const total = calculateTotal(payment);
    console.log(`${total} USD has been successfully wired to ${payment.name}`);
}

function calculateTotal(payment) {
    return payment.q1 + payment.q2 + payment.q3 + payment.q4;
}


module.exports = {
    processPayment,
    calculateTotal
};