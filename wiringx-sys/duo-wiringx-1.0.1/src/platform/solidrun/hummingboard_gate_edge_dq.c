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
#include "hummingboard_gate_edge_dq.h"

struct platform_t *hummingboardGateEdgeDQ = NULL;

/*
-------
  MIC
-------
| 0| 1| 2|0v|
| 3| 4| 5| 6|
| 7| 8| 9|10|
|11|12|13|14|
|15|16|17|18|
|19|20|21|22|

|23|24|25|26|
|27|28|29|30|
|31|32|5v|3v|
|--|
|IR|
|--|

--------
mikroBUS
--------

i	LABEL	PINFUNC			PAD			GPIO		wiringNo
1	AN		MB_AN			-
2	RST		POR_B			-
3	CS		ECSPI2_SS0		EIM_RW		GPIO2_IO26	33
4	SCK		ECSPI2_SCLK		EIM_CS0		GPIO2_IO23	34
5	MISO	ECSPI2_MISO		EIM_OE		GPIO2_IO25	35
6	MOSI	ECSPI2_MOSI		EIM_CS1		GPIO2_IO24	36
7	3V3		-
8	GND		-
9	PWM		PWM1_OUT		DISP0_DAT8	GPIO4_IO29	37
10	INT		ECSPI2_SS1		EIM_LBA		GPIO2_IO27	38
11	RX		UART3_TX_DATA	EIM_D24		GPIO3_IO24	39
12	TX		UART3_RX_DATA	EIM_D25		GPIO3_IO25	40
13	SCL		I2C3_SCL		EIM_D17		GPIO3_IO17	41
14	SDA		I2C3_SDA		EIM_D18		GPIO3_IO18	42
15	5V0		-
16	GND		-

*/

/*
 * Not all GPIO where usable through sysfs
 * from the kernel used.
 */
static int irq[] = {
	204, -1, 54,
	 91, 90, 94,
	 95, -1, -1,
	 -1, -1, -1,
	 -1, -1, -1,
	 -1, -1, -1,
	 -1, 70, -1,
	 -1, -1, -1,
	 67, -1, -1,
	 70, 71, 72,
	 73, -1, -1,
	 // TODO: mikroBUS
	 -1, -1, -1,
	 -1, -1, -1,
	 -1, -1, -1,
	 -1,
};

static int map[] = {
	/*	GPIO7_IO12,	GPIO7_IO11,	GPIO2_IO22 */
			195,				194,				 54,
	/*	GPIO3_IO27,	GPIO3_IO26,	GPIO3_IO30 */
			 91,				 90,				 94,
	/*	GPIO3_IO31,	GPIO5_IO04,	GPIO6_IO06 */
			 95,				125,				159,
	/*	GPIO2_IO16,	GPIO2_IO17,	GPIO2_IO18 */
			 48,				 47,				 50,
	/*	GPIO2_IO19,	GPIO2_IO20,	GPIO2_IO21 */
			 51,				 52,				 53,
	/*	GPIO2_IO28,	GPIO2_IO29,	GPIO3_IO00 */
			 60,				 61,				 64,
	/*	GPIO3_IO01,	GPIO3_IO12,	GPIO3_IO15 */
			 65,				 76,				 79,
	/*	GPIO3_IO14,	GPIO3_IO13,	GPIO3_IO02 */
			 78,				 77,				 66,
	/*	GPIO3_IO03,	GPIO3_IO04,	GPIO3_IO05 */
			 67,				 68,				 69,
	/*	GPIO3_IO06,	GPIO3_IO07,	GPIO3_IO08 */
			 70,				 71,				 72,
	/*	GPIO3_IO09,	GPIO3_IO11,	GPIO3_IO10 */
			 73,				 75,				 74,
	/* mikroBUS */
	/*	GPIO2_IO26,	GPIO2_IO23,	GPIO2_IO25 */
			 58,				 55,				 57,
	/*	GPIO2_IO24,	GPIO4_IO29,	GPIO2_IO27 */
			 56,				 120,				 59,
	/*	GPIO3_IO24,	GPIO3_IO25,	GPIO3_IO17 */
			 88,				 89,				 81,
	/*	GPIO3_IO18 */
			 82,
};

static int hummingboardGateEdgeDQValidGPIO(int pin) {
	if(pin == 1) {
		return -1;
	}
	if(pin >= 0 && pin < (sizeof(map)/sizeof(map[0]))) {
		return 0;
	} else {
		return -1;
	}
}

static int hummingboardGateEdgeDQISR(int i, enum isr_mode_t mode) {
	if(irq[i] == -1) {
		wiringXLog(LOG_ERR, "The %s gpio %d cannot be used as interrupt", hummingboardGateEdgeDQ->name[0], i);
		return -1;
	}
	return hummingboardGateEdgeDQ->soc->isr(i, mode);
}

static int hummingboardGateEdgeDQSetup(void) {
	hummingboardGateEdgeDQ->soc->setup();
	hummingboardGateEdgeDQ->soc->setMap(map, sizeof(map) / sizeof(map[0]));
	hummingboardGateEdgeDQ->soc->setIRQ(irq, sizeof(irq) / sizeof(irq[0]));
	return 0;
}

void hummingboardGateEdgeDQInit(void) {
	platform_register(&hummingboardGateEdgeDQ, "hummingboard_edge_dq");
	platform_add_alias(&hummingboardGateEdgeDQ, "hummingboard_gate_dq");

	hummingboardGateEdgeDQ->soc = soc_get("NXP", "IMX6DQRM");
	hummingboardGateEdgeDQ->soc->setMap(map, sizeof(map) / sizeof(map[0]));
	hummingboardGateEdgeDQ->soc->setIRQ(irq, sizeof(irq) / sizeof(irq[0]));

	hummingboardGateEdgeDQ->digitalRead = hummingboardGateEdgeDQ->soc->digitalRead;
	hummingboardGateEdgeDQ->digitalWrite = hummingboardGateEdgeDQ->soc->digitalWrite;
	hummingboardGateEdgeDQ->pinMode = hummingboardGateEdgeDQ->soc->pinMode;
	hummingboardGateEdgeDQ->setup = &hummingboardGateEdgeDQSetup;

	hummingboardGateEdgeDQ->isr = &hummingboardGateEdgeDQISR;
	hummingboardGateEdgeDQ->waitForInterrupt = hummingboardGateEdgeDQ->soc->waitForInterrupt;

	hummingboardGateEdgeDQ->selectableFd = hummingboardGateEdgeDQ->soc->selectableFd;
	hummingboardGateEdgeDQ->gc = hummingboardGateEdgeDQ->soc->gc;

	hummingboardGateEdgeDQ->validGPIO = &hummingboardGateEdgeDQValidGPIO;

}
