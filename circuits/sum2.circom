pragma circom 2.0.0;

template Sum(n) {
    signal input in[n];
    signal output out;
    signal intermediate[2*n];

    intermediate[0] <== in[0];

    for (var i=1; i<n; i++) {
        intermediate[i] <== in[i] + intermediate[i-1];
    }

    for (var i=0; i<n; i++) {
        intermediate[i + n] <== in[i] + intermediate[i - 1 + n];
    }
    out <== intermediate[2*n-1];
}

component main = Sum(50);
