pragma circom 2.0.0;

template Fib(n) {
    signal output out[n];
    out[0] <== 1;
    out[1] <== 1;
    for (var i=2; i<n; i++) {
        out[i] <== out[i-1] + out[i-2];
    }
}

component main = Fib(10);