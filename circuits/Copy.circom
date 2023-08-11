pragma circom 2.0.0;

template Copy(n) {
   signal input in[n];
   signal output out[n];
   for (var i=0; i<n; i++) {
      out[i] <== in[i];
   }
}

component main = Copy(20);