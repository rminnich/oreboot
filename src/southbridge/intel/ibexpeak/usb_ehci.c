/*
 * This file is part of the coreboot project.
 *
 * Copyright (C) 2008-2009 coresystems GmbH
 * Copyright (C) 2013 Vladimir Serbinenko
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation; version 2 of
 * the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
 */

#include <console/console.h>
#include <device/device.h>
#include <device/pci.h>
#include <device/pci_ids.h>
#include "pch.h"
#include <device/pci_ehci.h>
#include <arch/io.h>

static void usb_ehci_init(struct device *dev)
{
	u32 reg32;
	struct resource *res;
	u8 access_cntl;

	/* Disable Wake on Disconnect in RMH */
	reg32 = RCBA32(0x35b0);
	reg32 |= 0x22;
	RCBA32(0x35b0) = reg32;

	printk(BIOS_DEBUG, "EHCI: Setting up controller.. ");

	pci_write_config32(dev, 0x84, 0x130c8911);
	pci_write_config32(dev, 0x88, 0xa0);
	pci_write_config32(dev, 0xf4, 0x80808588);
	pci_write_config32(dev, 0xf4, 0x00808588);
	pci_write_config32(dev, 0xf4, 0x00808588);
	pci_write_config32(dev, 0xfc, 0x301b1728);

	reg32 = pci_read_config32(dev, PCI_COMMAND);
	reg32 |= PCI_COMMAND_MASTER;
	//reg32 |= PCI_COMMAND_SERR;
	pci_write_config32(dev, PCI_COMMAND, reg32);

	access_cntl = pci_read_config8(dev, 0x80);

	/* Enable writes to protected registers. */
	pci_write_config8(dev, 0x80, access_cntl | 1);

	res = find_resource(dev, PCI_BASE_ADDRESS_0);
	if (res) {
		/* Number of ports and companion controllers.  */
		reg32 = read32(res->base + 4);
		write32(res->base + 4, (reg32 & 0xfff00000) | 2);
	}

	/* Restore protection. */
	pci_write_config8(dev, 0x80, access_cntl);

	printk(BIOS_DEBUG, "done.\n");
}

static void usb_ehci_set_subsystem(device_t dev, unsigned vendor,
				   unsigned device)
{
	u8 access_cntl;

	access_cntl = pci_read_config8(dev, 0x80);

	/* Enable writes to protected registers. */
	pci_write_config8(dev, 0x80, access_cntl | 1);

	if (!vendor || !device) {
		pci_write_config32(dev, PCI_SUBSYSTEM_VENDOR_ID,
				   pci_read_config32(dev, PCI_VENDOR_ID));
	} else {
		pci_write_config32(dev, PCI_SUBSYSTEM_VENDOR_ID,
				   ((device & 0xffff) << 16) | (vendor &
								0xffff));
	}

	/* Restore protection. */
	pci_write_config8(dev, 0x80, access_cntl);
}


static struct pci_operations lops_pci = {
	.set_subsystem = &usb_ehci_set_subsystem,
};

static struct device_operations usb_ehci_ops = {
	.read_resources = pci_ehci_read_resources,
	.set_resources = pci_dev_set_resources,
	.enable_resources	= pci_dev_enable_resources,
	.init = usb_ehci_init,
	.scan_bus = 0,
	.ops_pci = &lops_pci,
};

static const unsigned short pci_device_ids[] = { 0x3b34, 0x3b3c, 0 };

static const struct pci_driver pch_usb_ehci __pci_driver = {
	.ops = &usb_ehci_ops,
	.vendor = PCI_VENDOR_ID_INTEL,
	.devices = pci_device_ids,
};
