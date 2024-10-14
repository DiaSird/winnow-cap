use winnow::{
    ascii::{multispace0, space0, till_line_ending},
    combinator::{alt, eof, opt},
    error::{ContextError, StrContext, StrContextValue},
    token::take_till,
    PResult, Parser,
};

#[derive(Debug)]
pub struct Rows<'a> {
    pub rows: Vec<(&'a str, &'a str)>,
}

pub fn till_parse_next_rows<'a>() -> impl Parser<&'a str, Rows<'a>, ContextError> {
    move |input: &mut &'a str| {
        let mut rows = Vec::new();

        loop {
            multispace0.parse_next(input)?;

            let row_item1 = take_till(0.., |c| c == ',')
                .context(StrContext::Expected(StrContextValue::StringLiteral(
                    "row_item",
                )))
                .parse_next(input)?;
            space0.parse_next(input)?;

            // split comma
            let res = opt(',').parse_next(input)?;
            if res.is_none() {
                break;
            };

            space0.parse_next(input)?;
            let row_item2 = till_line_ending
                .context(StrContext::Expected(StrContextValue::StringLiteral(
                    "row_item2",
                )))
                .parse_next(input)?;
            space0.parse_next(input)?;

            rows.push((row_item1, row_item2));
        }

        Ok(Rows { rows })
    }
}

#[derive(Debug)]
pub struct Columns<'a> {
    pub cols: Vec<Vec<&'a str>>,
}

pub fn till_parse_next_cols<'a>() -> impl Parser<&'a str, Columns<'a>, ContextError> {
    move |input: &mut &'a str| {
        let mut cols = Vec::new();

        loop {
            let mut col = vec![];
            loop {
                multispace0.parse_next(input)?;
                let col_item = take_till(0.., |c| c == ',' || c == '\n').parse_next(input)?;
                multispace0.parse_next(input)?;
                col.push(col_item);

                // split comma
                let res = opt(alt((',', '\n'))).parse_next(input)?;
                if res.is_none() {
                    break;
                };
            }

            cols.push(col);
            let is_eof: PResult<_, ContextError> = eof.parse_next(input);
            if is_eof.is_ok() {
                break;
            }
        }

        Ok(Columns { cols })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let input = include_str!("../templates/sample.csv");
        // dbg!(till_parse_next_rows().parse(input).unwrap());
        dbg!(till_parse_next_cols().parse(input).unwrap());
    }
}
