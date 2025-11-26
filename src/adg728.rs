
pub struct ADG728<I2C> {
    i2c: I2C,
    address: u8,
    enabled: u8,
}

impl<I2C> ADG728<I2C> {
    fn address_from_pins(a0: bool, a1: bool) -> u8 {
        0b1000100 | (a0 as u8) | ((a1 as u8) << 1)
    }
}

impl<I2C> ADG728<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self {
            i2c,
            address,
            enabled: 0,
        }
    }

    pub fn from_address_pins(i2c: I2C, a0: bool, a1: bool) -> Self {
        Self::new(i2c, Self::address_from_pins(a0, a1))
    }

    pub fn set_channels(&mut self, enabled: u8) -> Result<(), I2C::Error> {
        if self.enabled != enabled {
            self.i2c.write(self.address, &[self.enabled])?;
            self.enabled = enabled;
        }
        Ok(())
    }

    pub fn get_channels(&mut self) -> Result<u8, I2C::Error> {
        let mut buffer: [u8; 1] = [0];
        self.i2c.read(self.address, &mut buffer)?;
        self.enabled = buffer[0];
        Ok(self.enabled)
    }
}

impl<I2C> ADG728<I2C>
where
    I2C: embedded_hal_async::i2c::I2c,
{
    pub fn new_async(i2c: I2C, address: u8) -> Self {
        Self {
            i2c,
            address,
            enabled: 0,
        }
    }

    pub fn from_address_pins_async(i2c: I2C, a0: bool, a1: bool) -> Self {
        Self::new_async(i2c, Self::address_from_pins(a0, a1))
    }

    pub async fn set_channels_async(&mut self, enabled: u8) -> Result<(), I2C::Error> {
        if self.enabled != enabled {
            self.i2c.write(self.address, &[self.enabled]).await?;
            self.enabled = enabled;
        }
        Ok(())
    }

    pub async fn get_channels_async(&mut self) -> Result<u8, I2C::Error> {
        let mut buffer: [u8; 1] = [0];
        self.i2c.read(self.address, &mut buffer).await?;
        self.enabled = buffer[0];
        Ok(self.enabled)
    }
}

impl<I2C> core::fmt::Debug for ADG728<I2C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ADG728")
            .field("address", &self.address)
            .field("enabled_mask", &self.enabled)
            .finish()
    }
}
