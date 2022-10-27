#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

typedef struct raptorq_encoder raptorq_encoder_t;
typedef struct raptorq_decoder raptorq_decoder_t;

extern raptorq_encoder_t *raptorq_encoder_with_defaults(const uint8_t *data, size_t data_len, uint16_t maximum_transmission_unit);

extern size_t raptorq_encoder_encode(raptorq_encoder_t *encoder, repair_packets_per_block);

extern size_t raptorq_encoder_next_packet(raptorq_encoder_t *encoder, uint8_t *output, size_t output_len);

extern void raptorq_encoder_free(raptorq_encoder_t *encoder);

extern raptorq_decoder_t *raptorq_decoder_with_defaults(uint64_t transfer_length, uint16_t maximum_transmission_unit);

extern size_t raptorq_decoder_decode(raptorq_decoder_t *decoder, uint8_t *packet, size_t packet_len, uint8_t *output, size_t output_len);

extern void raptorq_decoder_free(raptorq_decoder_t *decoder);
