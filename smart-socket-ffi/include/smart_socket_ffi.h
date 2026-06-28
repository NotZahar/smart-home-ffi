#ifndef SMART_SOCKET_FFI_H
#define SMART_SOCKET_FFI_H

#ifdef __cplusplus
extern "C" {
#endif

typedef enum SmartSocketState {
    SMART_SOCKET_STATE_OFF = 0,
    SMART_SOCKET_STATE_ON = 1,
} SmartSocketState;

typedef enum SmartSocketStatus {
    SMART_SOCKET_STATUS_OK = 0,
    SMART_SOCKET_STATUS_NULL_POINTER = 1,
    SMART_SOCKET_STATUS_INVALID_ARGUMENT = 2,
} SmartSocketStatus;

typedef struct SmartSocketHandle SmartSocketHandle;

SmartSocketHandle *smart_socket_create(float default_active_power, float max_power_offset);
SmartSocketHandle *smart_socket_default(void);
void smart_socket_destroy(SmartSocketHandle *socket);

SmartSocketStatus smart_socket_turn_on(SmartSocketHandle *socket);
SmartSocketStatus smart_socket_turn_off(SmartSocketHandle *socket);
SmartSocketStatus smart_socket_get_state(
    const SmartSocketHandle *socket,
    SmartSocketState *state_out
);
SmartSocketStatus smart_socket_get_power(SmartSocketHandle *socket, float *power_out);

#ifdef __cplusplus
}
#endif

#endif
