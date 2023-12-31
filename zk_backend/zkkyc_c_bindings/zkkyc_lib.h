#ifndef ZKKYC_LIB_H
#define ZKKYC_LIB_H

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

const unsigned char *create_proof(unsigned long user_age,
                                  unsigned long user_country,
                                  unsigned long public_age,
                                  const unsigned long *public_countries,
                                  unsigned long public_countries_len,
                                  unsigned long *proof_len);

unsigned char verify_proof(unsigned long public_age,
                           const unsigned long *public_countries,
                           unsigned long public_countries_len,
                           const unsigned char *proof,
                           unsigned long proof_len);

#endif /* ZKKYC_LIB_H */
