/*
	Copyright (c) 2016 CurlyMo <curlymoo1@gmail.com>

  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

#include <sys/mman.h>
#include <unistd.h>
#include <stdio.h>
#include <fcntl.h>
#include <errno.h>
#include <string.h>
#include <stdlib.h>
#include <sys/ioctl.h>
#include <signal.h>

#include "../../soc/soc.h"
#include "../../wiringx.h"
#include "../platform.h"
#include "raspberrypi2.h"

struct platform_t *raspberrypi2 = NULL;

static int map[] = {
	/* 	FSEL17,	FSEL18,	FSEL27,	FSEL22 	*/
			17, 		18, 		27, 		22,
	/* 	FSEL23,	FSEL24,	FSEL25,	FSEL4 	*/
			23, 		24, 		25, 		 4,
	/* 	FSEL2,	FSEL3,	FSEL8,	FSEL7 	*/
			 2, 		 3, 		 8, 		 7,
	/*	FSEL10,	FSEL9,	FSEL11,	FSEL14	*/
			10,			 9,			11,			14,
	/*	FSEL15													*/
			15,			-1,			-1,			-1,
	/*					FSEL5,	FSEL6,	FSEL13	*/
			-1,			 5,			 6,			13,
	/*	FSEL19,	FSEL26,	FSEL12,	FSEL16	*/
			19,			26,			12,			16,
	/*	FSEL20,	FSEL21,	FSEL0,	FSEL1		*/
			20,			21,			 0,			 1
};

static int raspberrypi2ValidGPIO(int pin) {
	if(pin >= 0 && pin < (sizeof(map)/sizeof(map[0]))) {
		if(map[pin] == -1) {
			return -1;
		}
		return 0;
	} else {
		return -1;
	}
}

static int raspberrypi2Setup(void) {
	const size_t size = sizeof(map) / sizeof(map[0]);
	raspberrypi2->soc->setup();
	raspberrypi2->soc->setMap(map, size);
	raspberrypi2->soc->setIRQ(map, size);
	return 0;
}

void raspberrypi2Init(void) {
	platform_register(&raspberrypi2, "raspberrypi2");

	raspberrypi2->soc = soc_get("Broadcom", "2836");
	raspberrypi2->soc->setMap(map, sizeof(map) / sizeof(map[0]));

	raspberrypi2->digitalRead = raspberrypi2->soc->digitalRead;
	raspberrypi2->digitalWrite = raspberrypi2->soc->digitalWrite;
	raspberrypi2->pinMode = raspberrypi2->soc->pinMode;
	raspberrypi2->setup = raspberrypi2Setup;

	raspberrypi2->isr = raspberrypi2->soc->isr;
	raspberrypi2->waitForInterrupt = raspberrypi2->soc->waitForInterrupt;

	raspberrypi2->selectableFd = raspberrypi2->soc->selectableFd;
	raspberrypi2->gc = raspberrypi2->soc->gc;

	raspberrypi2->validGPIO = &raspberrypi2ValidGPIO;

}
