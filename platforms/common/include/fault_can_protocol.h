/**
 * @file fault_can_protocol.h
 * @brief Fault CAN Protocol.
 *
 */


#ifndef _OSCC_FAULT_CAN_PROTOCOL_H_
#define _OSCC_FAULT_CAN_PROTOCOL_H_


#include <stdint.h>


/*
 * @brief Fault report message (CAN frame) ID.
 *
 */
#define OSCC_FAULT_REPORT_CAN_ID (0x99)

/*
 * @brief Fault report message (CAN frame) length.
 *
 */
#define OSCC_FAULT_REPORT_CAN_DLC (8)


typedef enum
{
    FAULT_ORIGIN_BRAKE,
    FAULT_ORIGIN_STEERING,
    FAULT_ORIGIN_THROTTLE
} fault_origin_id_t;


/**
 * @brief Fault report message data.
 *
 * Message size (CAN frame DLC): \ref OSCC_MODULE_FAULT_REPORT_CAN_DLC
 *
 */
typedef struct
{
    uint32_t fault_origin_id; /* ID of the module that is sending out the fault. */

    uint8_t reserved[4]; /* Reserved */
} oscc_module_fault_report_s;


#endif /* _OSCC_FAULT_CAN_PROTOCOL_H_ */
