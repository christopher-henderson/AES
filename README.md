# AES

This crate implements AES-256 as defined by FIPS 197.

AES is a 128-bit block cipher originally called Rijndael.

The NIST publication describing AES can be found on the [NIST website](https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.197.pdf)

Unit tests are provided, including step-by-step test vectors found in FIPS 197 C.3

## Should I `extern` this crate?
![Crypto Hipster](https://raw.githubusercontent.com/christopher-henderson/AES/master/2fppk8.jpg)

Please don't. _PLEASE_ don't. I've had plenty of fun reading and implementing this, and it does indeed pass the test vectors listed in FIPS 197, but I just cannot overemphasize how much of a bad idea it would be to use this for anything other than didactic purposes. Let us count all of the ways that this is a bad idea:

* Endianness is a concern in AES, but endianness is _NOT_ yet addressed by this code.
* I'm not satisified with the interface for `encrypt` and `decrypt`. Given that these are the _only_ exported in functions, and that I play around with this on the weekends with little regard for anyone else, I think it's safe to call this interface "unstable".
* AES is a _block_ cipher. Although, yes, this is indeed AES what you are probably looking for is AES operating in a _counter mode_, such as Galois Counter Mode.
* And even then, while I do have plans (and a branch open) to implement GCM, all ciphers benefit chiefly from the unblinking attention of an army of programmers and scientists. Meanwhile, I'm surprised you even found this repo.

So if you want to read some code, or even give me a code review, yay! If you want me to publish this to crates.io, then HAHAHAHAHAHAHAHAHAHAH....
