use bytes::{BufMut, Buf};

pub(super) trait Sniffer {
    type SniffError;
    fn skip(&mut self, count: usize) -> Result<(), Self::SniffError>;
    fn sniff_to<B: BufMut>(&mut self, to: &mut B) -> Result<(), Self::SniffError>;

    fn sniff_byte(&mut self) -> Result<u8, Self::SniffError> {
        let mut byte = [0u8; 1];
        self.sniff_to(&mut byte.as_mut_slice())?;
        Ok(byte[0])
    }
    fn sniff_pod<T: bytemuck::Pod + Copy>(&mut self) -> Result<T, Self::SniffError> {
        self.sniff_pod_ext::<_, false>()
    }
    fn sniff_pod_reverse<T: bytemuck::Pod + Copy>(&mut self) -> Result<T, Self::SniffError> {
        self.sniff_pod_ext::<_, true>()
    }
    fn sniff_pod_ext<T: bytemuck::Pod + Copy, const REVERSE: bool>(
        &mut self,
    ) -> Result<T, Self::SniffError> {
        let mut val: T = bytemuck::Zeroable::zeroed();
        let to = bytemuck::bytes_of_mut(&mut val);
        self.sniff_to(&mut &mut *to)?;
        if to.len() > 1 && REVERSE {
            to.reverse();
        }
        Ok(val)
    }
    fn sniff_vec(&mut self, len: usize) -> Result<Vec<u8>, Self::SniffError> {
        let mut vec = Vec::with_capacity(len).limit(len);
        self.sniff_to(&mut vec)?;
        Ok(vec.into_inner())
    }
    fn sniff_pod_vec_reverse<T: bytemuck::Pod + Copy>(&mut self, len: usize) -> Result<Vec<T>, Self::SniffError> {
        self.sniff_pod_vec_ext::<_, true>(len)
    }
    fn sniff_pod_vec_ext<T: bytemuck::Pod + Copy, const REVERSE: bool>(&mut self, len: usize) -> Result<Vec<T>, Self::SniffError> {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(self.sniff_pod_ext::<_, REVERSE>()?);
        }
        Ok(vec)
    }
}

#[derive(Clone)]
pub(super) struct F2Sniffer<'a> {
    buf: &'a [u8],
}
impl<'a> F2Sniffer<'a> {
    pub(super) fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }
    pub(super) fn len(&self) -> usize {
        self.buf.len()
    }
    pub(super) fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}

#[derive(Debug)]
pub enum F2SniffError {
    NotEnoughBytes,
}

impl<'a> Sniffer for F2Sniffer<'a> {
    type SniffError = F2SniffError;
    fn skip(&mut self, count: usize) -> Result<(), Self::SniffError> {
        if self.buf.len() < count {
            return Err(F2SniffError::NotEnoughBytes);
        }
        self.buf.advance(count);
        Ok(())
    }
    fn sniff_to<B: BufMut>(&mut self, to: &mut B) -> Result<(), Self::SniffError> {
        let len = to.remaining_mut();
        if self.buf.len() < len {
            return Err(F2SniffError::NotEnoughBytes);
        }
        to.put_slice(&self.buf[..len]);
        self.buf.advance(len);
        Ok(())
    }
}
