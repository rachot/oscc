#include <Arduino.h>
#include <stdint.h>

#include "globals.h"
#include "accumulator.h"
#include "helper.h"


void accumulator_init( void )
{
    pinMode( PIN_ACCUMULATOR_PUMP_MOTOR, OUTPUT );

    accumulator_turn_pump_off( );
}


void accumulator_turn_pump_off( void )
{
    digitalWrite( PIN_ACCUMULATOR_PUMP_MOTOR, LOW );
}


void accumulator_turn_pump_on( void )
{
    digitalWrite( PIN_ACCUMULATOR_PUMP_MOTOR, HIGH );
}


void accumulator_maintain_pressure( void )
{
    const float accumulator_alpha = 0.05;

    uint16_t raw_accumulator_data = analogRead( PIN_ACCUMULATOR_PRESSURE_SENSOR );

    float pressure = raw_adc_to_pressure( raw_accumulator_data );

    brake_state.accumulator_pressure =
        ( accumulator_alpha * pressure ) +
        ( ( 1.0 - accumulator_alpha ) * brake_state.accumulator_pressure );

    if ( brake_state.accumulator_pressure < PARAM_ACCUMULATOR_PRESSURE_MIN_IN_DECIBARS )
    {
        accumulator_turn_pump_on( );
    }

    if ( brake_state.accumulator_pressure > PARAM_ACCUMULATOR_PRESSURE_MAX_IN_DECIBARS )
    {
        accumulator_turn_pump_off( );
    }
}
