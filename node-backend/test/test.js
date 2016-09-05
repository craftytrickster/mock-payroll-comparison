'use strict';

const expect = require('expect.js');
const calculateTotal = require('../src/submission').calculateTotal;

describe('Payment Calculation', () => {
  it('should aggregate amount correctly', () => {
    const samplePayment = {
        name: 'Weyland-Yutani Corporation',
        q1: 30000,
        q2: 40000,
        q3: 20000,
        q4: 0
    };

    const expectedTotal = 90000;
    const actualTotal = calculateTotal(samplePayment);

    expect(expectedTotal).to.equal(actualTotal);
  });
});
