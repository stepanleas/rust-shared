#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum OrderStatus {
    #[default]
    Pending,
    Paid,
    Approved,
    Cancelling,
    Canceled,
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum PaymentOrderStatus {
    #[default]
    Pending,
    Cancelled,
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum PaymentStatus {
    #[default]
    Completed,
    Cancelled,
    Failed,
}