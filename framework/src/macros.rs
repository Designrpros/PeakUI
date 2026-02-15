#[macro_export]
macro_rules! vstack {
    ($($child:expr),* $(,)?) => {
        $crate::layout::VStack::new()
            $(.push($child))*
    };

    (spacing: $spacing:expr, $($child:expr),* $(,)?) => {
        $crate::layout::VStack::new().spacing($spacing)
            $(.push($child))*
    };
}

#[macro_export]
macro_rules! hstack {
    ($($child:expr),* $(,)?) => {
        $crate::layout::HStack::new()
            $(.push($child))*
    };

    (spacing: $spacing:expr, $($child:expr),* $(,)?) => {
        $crate::layout::HStack::new().spacing($spacing)
            $(.push($child))*
    };
}

#[macro_export]
macro_rules! zstack {
    ($($child:expr),* $(,)?) => {
        $crate::layout::ZStack::new()
            $(.push($child))*
    };
}

#[macro_export]
macro_rules! grid {
    (cols: $cols:expr, $($child:expr),* $(,)?) => {
        $crate::layout::ResponsiveGrid::new().columns($cols)
            $(.push($child))*
    };
}
