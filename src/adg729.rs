pub struct ADG729<I2C> {
    i2c: I2C,
    address: u8,
    enabled: u8,
}

impl<I2C> ADG729<I2C>
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

    pub fn set_channels(&mut self, enabled: u8) -> Result<(), I2C::Error> {
        if self.enabled != enabled {
            self.i2c.write(self.address, &[self.enabled])?;
            self.enabled = enabled;
        }
        Ok(())
    }

    pub fn set_a(&mut self, enabled: u8) -> Result<(), I2C::Error> {
        self.set_channels((self.enabled & 0xF0) | ((enabled & 0xF) << 0))
    }

    pub fn set_b(&mut self, enabled: u8) -> Result<(), I2C::Error> {
        self.set_channels((self.enabled & 0x0F) | ((enabled & 0xF) << 4))
    }

    pub fn get_channels(&mut self) -> Result<u8, I2C::Error> {
        let mut buffer: [u8; 1] = [0];
        self.i2c.read(self.address, &mut buffer)?;
        self.enabled = buffer[0];
        Ok(self.enabled)
    }

    pub fn get_a(&mut self) -> Result<u8, I2C::Error> {
        Ok((self.get_channels()? >> 0) & 0x0F)
    }

    pub fn get_b(&mut self) -> Result<u8, I2C::Error> {
        Ok((self.get_channels()? >> 4) & 0x0F)
    }
}

impl<I2C> ADG729<I2C>
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

    pub async fn set_channels_async(&mut self, enabled: u8) -> Result<(), I2C::Error> {
        if self.enabled != enabled {
            self.i2c.write(self.address, &[self.enabled]).await?;
            self.enabled = enabled;
        }
        Ok(())
    }

    pub async fn set_a_async(&mut self, enabled: u8) -> Result<(), I2C::Error> {
        self.set_channels_async((self.enabled & 0xF0) | ((enabled & 0xF) << 0))
            .await
    }

    pub async fn set_b_async(&mut self, enabled: u8) -> Result<(), I2C::Error> {
        self.set_channels_async((self.enabled & 0x0F) | ((enabled & 0xF) << 4))
            .await
    }

    pub async fn get_channels_async(&mut self) -> Result<u8, I2C::Error> {
        let mut buffer: [u8; 1] = [0];
        self.i2c.read(self.address, &mut buffer).await?;
        self.enabled = buffer[0];
        Ok(self.enabled)
    }

    pub async fn get_a_async(&mut self) -> Result<u8, I2C::Error> {
        Ok((self.get_channels_async().await? >> 0) & 0x0F)
    }

    pub async fn get_b_async(&mut self) -> Result<u8, I2C::Error> {
        Ok((self.get_channels_async().await? >> 4) & 0x0F)
    }
}

impl<I2C> core::fmt::Debug for ADG729<I2C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ADG729")
            .field("address", &self.address)
            .field("enabled", &self.enabled)
            .finish()
    }
}
