pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 && year % 400 == 0{
        return true;
    }
    if year % 4 == 0 && year % 100 !=0{
        return true;
    }
    return false;
}
// https://exercism.io/my/solutions/7f808816cbcf44bc984c3546d9143125
