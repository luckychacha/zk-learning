pragma circom 2.0.0;

template SumOut(n) {
    signal input in[n];
    signal output out[n];
    out[0] <== in[0];
    for (var i=1; i<n; i++) {
        out[i] <== in[i] + out[i-1];
    }
}
component main = SumOut(18);