// Package weather provides weather condition for location.
package weather

// CurrentCondition stores info about weather for current location.
var CurrentCondition string

// CurrentLocation stores current location.
var CurrentLocation string

// Forecast gives weather condition for location.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
