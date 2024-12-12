program AoC24_D6
    implicit none
    integer, parameter :: field_size = 140
    character(len=field_size) :: line
    character(len=10) :: arg
    character :: field(field_size,field_size)
    character :: current
    integer :: area, multiplier, read_unit, sum, x, y
    logical :: part2

    area = 0
    multiplier = 0
    sum = 0
    part2 = .false.
    if (command_argument_count() == 1) then
        call get_command_argument(1, arg)
        part2 = arg == "--part2"
    end if

    open(read_unit, file="inputs/12.txt", status="old", action="read")
    do y = 1, field_size
        read(read_unit, '(A)') line
        do x = 1, field_size
            field(x, y) = line(x:x)
        end do
    end do
    close(read_unit)

    do y = 1, field_size
        do x = 1, field_size
            current = field(x,y)
            if (current == '_') then
                cycle
            end if
            if(.not. part2) then
                call fill(x, y)
            else
                call fill2(x, y)
            end if
            call clear(x, y)
            sum = sum + area * multiplier
            area = 0
            multiplier = 0
        end do
    end do
    print *, sum/2

contains
    recursive subroutine fill(x, y)
        integer, intent(in) :: x, y
        character :: next

        field(x,y) = '.'
        area = area + 1
        
        if(x>1) then
            next = field(x-1,y)
            if(next == current) then
                call fill(x-1, y)
            else if (next /= '.') then
                multiplier = multiplier + 1
            end if
        else
            multiplier = multiplier + 1
        end if
        if(x<field_size) then
            next = field(x+1,y)
            if(next == current) then
                call fill(x+1, y)
            else if (next /= '.') then
                multiplier = multiplier + 1
            end if
        else
            multiplier = multiplier + 1
        end if
        if(y>1) then
            next = field(x,y-1)
            if(next == current) then
                call fill(x, y-1)
            else if (next /= '.') then
                multiplier = multiplier + 1
            end if
        else
            multiplier = multiplier + 1
        end if
        if(y<field_size) then
            next = field(x,y+1)
            if(next == current) then
                call fill(x, y+1)
            else if (next /= '.') then
                multiplier = multiplier + 1
            end if
        else
            multiplier = multiplier + 1
        end if
    end subroutine fill

    recursive subroutine fill2(x, y)
        integer, intent(in) :: x, y
        logical :: left, right, top, down, top_left, top_right, down_left, down_right

        field(x,y) = '.'
        area = area + 1
        left = x==1 .or. field(x-1,y) /= '.' .and. field(x-1,y) /= current
        right = x==field_size .or. field(x+1,y) /= '.' .and. field(x+1,y) /= current
        top = y==1 .or. field(x,y-1) /= '.' .and. field(x,y-1) /= current
        down = y==field_size .or. field(x,y+1) /= '.' .and. field(x,y+1) /= current
        top_left = x==1 .and. y==1 .or. &
            x/=1 .and. y/=1 .and. field(x-1,y-1) /= '.' .and. field(x-1,y-1) /= current
        top_right = x==field_size .and. y==1 .or. &
            x/=field_size .and. y/=1 .and. field(x+1,y-1) /= '.' .and. field(x+1,y-1) /= current
        down_left = x==1 .and. y==field_size .or. &
            x/=1 .and. y/=field_size .and. field(x-1,y+1) /= '.' .and. field(x-1,y+1) /= current
        down_right = x==field_size .and. y==field_size .or. &
            x/=field_size .and. y/=field_size .and. field(x+1,y+1) /= '.' .and. field(x+1,y+1) /= current

        if(top .and. left .or. .not. top .and. .not. left .and. top_left) then
            multiplier = multiplier + 1
        end if
        if(top .and. right .or. .not. top .and. .not. right .and. top_right) then
            multiplier = multiplier + 1
        end if
        if(down .and. left .or. .not. down .and. .not. left .and. down_left) then
            multiplier = multiplier + 1
        end if
        if(down .and. right .or. .not. down .and. .not. right .and. down_right) then
            multiplier = multiplier + 1
        end if

        if(x /= 1 .and. field(x-1,y) == current) then
            call fill2(x-1,y)
        end if
        if(x /= field_size .and. field(x+1,y) == current) then
            call fill2(x+1,y)
        end if
        if(y /= 1 .and. field(x,y-1) == current) then
            call fill2(x,y-1)
        end if
        if(y /= field_size .and. field(x,y+1) == current) then
            call fill2(x,y+1)
        end if
    end subroutine fill2

    recursive subroutine clear(x, y)
        integer, intent(in) :: x, y
        character :: next

        field(x,y) = '_'
        area = area + 1
        
        if(x>1 .and. field(x-1, y) == '.') then
            call clear(x-1, y)
        end if
        if(x<field_size .and. field(x+1, y) == '.') then
            call clear(x+1, y)
        end if
        if(y>1 .and. field(x, y-1) == '.') then
            call clear(x, y-1)
        end if
        if(y<field_size .and. field(x, y+1) == '.') then
            call clear(x, y+1)
        end if
    end subroutine clear
end program AoC24_D6
