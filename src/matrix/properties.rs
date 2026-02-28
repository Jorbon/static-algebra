use crate::{Iterable, Matrix, Mat0, Number, StaticList};


impl<T, Row: StaticList<T>> From<()> for Mat0<T, Row> {
    #[inline]
    fn from(_value: ()) -> Self {
        Mat0::VALUE
    }
}

impl<T, Row: StaticList<T>, Inner: StaticList<Row>, IntoRow: Into<Row>, IntoInner: Into<Inner>> From<(IntoInner, IntoRow)> for Matrix<T, Row, Inner> {
    #[inline]
    fn from(value: (IntoInner, IntoRow)) -> Self {
        Self::with_inner(value.0.into(), value.1.into())
    }
}


impl<'a, T, Row: StaticList<T>, Inner: StaticList<Row>> core::fmt::Debug for Matrix<T, Row, Inner>
where
    T: core::fmt::Debug + 'a,
    Row: Iterable<T>,
    Inner: Iterable<Row>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let rows = <Self as StaticList<Row>>::Length::VALUE;
        let columns = <Row as StaticList<T>>::Length::VALUE;
        
        write!(f, "Mat{rows}")?;
        if columns != rows {
            write!(f, "x{columns}")?;
        }
        writeln!(f, " (")?;
        
        for row in self.iter() {
            write!(f, "\t[\t")?;
            for (i, value) in row.iter().enumerate() {
                if i > 0 { write!(f, ",\t")?; }
                write!(f, "{:?}", value)?;
            }
            writeln!(f, "\t],")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<'a, T, Row: StaticList<T>, Inner: StaticList<Row>> core::fmt::Display for Matrix<T, Row, Inner>
where
    T: core::fmt::Display + 'a,
    Row: Iterable<T>,
    Inner: Iterable<Row>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let rows = <Self as StaticList<Row>>::Length::VALUE;
        let columns = <Row as StaticList<T>>::Length::VALUE;
        
        write!(f, "Mat{rows}")?;
        if columns != rows {
            write!(f, "x{columns}")?;
        }
        writeln!(f, " (")?;
        
        for row in self.iter() {
            write!(f, "\t[\t")?;
            for (i, value) in row.iter().enumerate() {
                if i > 0 { write!(f, ",\t")?; }
                write!(f, "{}", value)?;
            }
            writeln!(f, "\t],")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

