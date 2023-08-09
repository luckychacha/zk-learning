pragma circom 2.0.0;

template Sum(n) {
    signal input in[n];
    signal output out;
    signal intermediate[n];

    intermediate[0] <== in[0];

    for (var i=1; i<n; i++) {
        intermediate[i] <== in[i] + intermediate[i-1];
    }
    out <== intermediate[n-1];
}

component main = Sum(50000);
