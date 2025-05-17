use lucide_yew::{Component, *};
use yew::prelude::*;
#[function_component]
pub fn Icons() -> Html {
    html! {
        <div class="w-full max-w-80 py-4">
            <IconsA />
            <IconsB />
            <IconsC />
            <IconsD />
            <IconsE />
            <IconsF />
            <IconsG />
            <IconsH />
            <IconsI />
            <IconsJ />
            <IconsK />
            <IconsL />
            <IconsM />
            <IconsN />
            <IconsO />
            <IconsP />
            <IconsQ />
            <IconsR />
            <IconsS />
            <IconsT />
            <IconsU />
            <IconsV />
            <IconsW />
            <IconsX />
            <IconsY />
            <IconsZ />
        </div>
    }
}
#[function_component]
pub fn IconsA() -> Html {
    let icons = [
        (html! { <AArrowDown /> }, "A Arrow Down"),
        (html! { <AArrowUp /> }, "A Arrow Up"),
        (html! { <ALargeSmall /> }, "A Large Small"),
        (html! { <Accessibility /> }, "Accessibility"),
        (html! { <Activity /> }, "Activity"),
        (html! { <AirVent /> }, "Air Vent"),
        (html! { <Airplay /> }, "Airplay"),
        (html! { <AlarmClock /> }, "Alarm Clock"),
        (html! { <AlarmClockCheck /> }, "Alarm Clock Check"),
        (html! { <AlarmClockMinus /> }, "Alarm Clock Minus"),
        (html! { <AlarmClockOff /> }, "Alarm Clock Off"),
        (html! { <AlarmClockPlus /> }, "Alarm Clock Plus"),
        (html! { <AlarmSmoke /> }, "Alarm Smoke"),
        (html! { <Album /> }, "Album"),
        (html! { <AlignCenter /> }, "Align Center"),
        (
            html! { <AlignCenterHorizontal /> },
            "Align Center Horizontal",
        ),
        (html! { <AlignCenterVertical /> }, "Align Center Vertical"),
        (html! { <AlignEndHorizontal /> }, "Align End Horizontal"),
        (html! { <AlignEndVertical /> }, "Align End Vertical"),
        (
            html! { <AlignHorizontalDistributeCenter /> },
            "Align Horizontal Distribute Center",
        ),
        (
            html! { <AlignHorizontalDistributeEnd /> },
            "Align Horizontal Distribute End",
        ),
        (
            html! { <AlignHorizontalDistributeStart /> },
            "Align Horizontal Distribute Start",
        ),
        (
            html! { <AlignHorizontalJustifyCenter /> },
            "Align Horizontal Justify Center",
        ),
        (
            html! { <AlignHorizontalJustifyEnd /> },
            "Align Horizontal Justify End",
        ),
        (
            html! { <AlignHorizontalJustifyStart /> },
            "Align Horizontal Justify Start",
        ),
        (
            html! { <AlignHorizontalSpaceAround /> },
            "Align Horizontal Space Around",
        ),
        (
            html! { <AlignHorizontalSpaceBetween /> },
            "Align Horizontal Space Between",
        ),
        (html! { <AlignJustify /> }, "Align Justify"),
        (html! { <AlignLeft /> }, "Align Left"),
        (html! { <AlignRight /> }, "Align Right"),
        (html! { <AlignStartHorizontal /> }, "Align Start Horizontal"),
        (html! { <AlignStartVertical /> }, "Align Start Vertical"),
        (
            html! { <AlignVerticalDistributeCenter /> },
            "Align Vertical Distribute Center",
        ),
        (
            html! { <AlignVerticalDistributeEnd /> },
            "Align Vertical Distribute End",
        ),
        (
            html! { <AlignVerticalDistributeStart /> },
            "Align Vertical Distribute Start",
        ),
        (
            html! { <AlignVerticalJustifyCenter /> },
            "Align Vertical Justify Center",
        ),
        (
            html! { <AlignVerticalJustifyEnd /> },
            "Align Vertical Justify End",
        ),
        (
            html! { <AlignVerticalJustifyStart /> },
            "Align Vertical Justify Start",
        ),
        (
            html! { <AlignVerticalSpaceAround /> },
            "Align Vertical Space Around",
        ),
        (
            html! { <AlignVerticalSpaceBetween /> },
            "Align Vertical Space Between",
        ),
        (html! { <Ambulance /> }, "Ambulance"),
        (html! { <Ampersand /> }, "Ampersand"),
        (html! { <Ampersands /> }, "Ampersands"),
        (html! { <Amphora /> }, "Amphora"),
        (html! { <Anchor /> }, "Anchor"),
        (html! { <Angry /> }, "Angry"),
        (html! { <Annoyed /> }, "Annoyed"),
        (html! { <Antenna /> }, "Antenna"),
        (html! { <Anvil /> }, "Anvil"),
        (html! { <Aperture /> }, "Aperture"),
        (html! { <AppWindow /> }, "App Window"),
        (html! { <AppWindowMac /> }, "App Window Mac"),
        (html! { <Apple /> }, "Apple"),
        (html! { <Archive /> }, "Archive"),
        (html! { <ArchiveRestore /> }, "Archive Restore"),
        (html! { <ArchiveX /> }, "Archive X"),
        (html! { <Armchair /> }, "Armchair"),
        (html! { <ArrowBigDown /> }, "Arrow Big Down"),
        (html! { <ArrowBigDownDash /> }, "Arrow Big Down Dash"),
        (html! { <ArrowBigLeft /> }, "Arrow Big Left"),
        (html! { <ArrowBigLeftDash /> }, "Arrow Big Left Dash"),
        (html! { <ArrowBigRight /> }, "Arrow Big Right"),
        (html! { <ArrowBigRightDash /> }, "Arrow Big Right Dash"),
        (html! { <ArrowBigUp /> }, "Arrow Big Up"),
        (html! { <ArrowBigUpDash /> }, "Arrow Big Up Dash"),
        (html! { <ArrowDown /> }, "Arrow Down"),
        (html! { <ArrowDown01 /> }, "Arrow Down 01"),
        (html! { <ArrowDown10 /> }, "Arrow Down 10"),
        (html! { <ArrowDownAZ /> }, "Arrow Down Az"),
        (html! { <ArrowDownFromLine /> }, "Arrow Down From Line"),
        (html! { <ArrowDownLeft /> }, "Arrow Down Left"),
        (html! { <ArrowDownNarrowWide /> }, "Arrow Down Narrow Wide"),
        (html! { <ArrowDownRight /> }, "Arrow Down Right"),
        (html! { <ArrowDownToDot /> }, "Arrow Down To Dot"),
        (html! { <ArrowDownToLine /> }, "Arrow Down To Line"),
        (html! { <ArrowDownUp /> }, "Arrow Down Up"),
        (html! { <ArrowDownWideNarrow /> }, "Arrow Down Wide Narrow"),
        (html! { <ArrowDownZA /> }, "Arrow Down Za"),
        (html! { <ArrowLeft /> }, "Arrow Left"),
        (html! { <ArrowLeftFromLine /> }, "Arrow Left From Line"),
        (html! { <ArrowLeftRight /> }, "Arrow Left Right"),
        (html! { <ArrowLeftToLine /> }, "Arrow Left To Line"),
        (html! { <ArrowRight /> }, "Arrow Right"),
        (html! { <ArrowRightFromLine /> }, "Arrow Right From Line"),
        (html! { <ArrowRightLeft /> }, "Arrow Right Left"),
        (html! { <ArrowRightToLine /> }, "Arrow Right To Line"),
        (html! { <ArrowUp /> }, "Arrow Up"),
        (html! { <ArrowUp01 /> }, "Arrow Up 01"),
        (html! { <ArrowUp10 /> }, "Arrow Up 10"),
        (html! { <ArrowUpAZ /> }, "Arrow Up Az"),
        (html! { <ArrowUpDown /> }, "Arrow Up Down"),
        (html! { <ArrowUpFromDot /> }, "Arrow Up From Dot"),
        (html! { <ArrowUpFromLine /> }, "Arrow Up From Line"),
        (html! { <ArrowUpLeft /> }, "Arrow Up Left"),
        (html! { <ArrowUpNarrowWide /> }, "Arrow Up Narrow Wide"),
        (html! { <ArrowUpRight /> }, "Arrow Up Right"),
        (html! { <ArrowUpToLine /> }, "Arrow Up To Line"),
        (html! { <ArrowUpWideNarrow /> }, "Arrow Up Wide Narrow"),
        (html! { <ArrowUpZA /> }, "Arrow Up Za"),
        (html! { <ArrowsUpFromLine /> }, "Arrows Up From Line"),
        (html! { <Asterisk /> }, "Asterisk"),
        (html! { <AtSign /> }, "At Sign"),
        (html! { <Atom /> }, "Atom"),
        (html! { <AudioLines /> }, "Audio Lines"),
        (html! { <AudioWaveform /> }, "Audio Waveform"),
        (html! { <Award /> }, "Award"),
        (html! { <Axe /> }, "Axe"),
        (html! { <Axis3D /> }, "Axis 3 D"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsB() -> Html {
    let icons = [
        (html! { <Baby /> }, "Baby"),
        (html! { <Backpack /> }, "Backpack"),
        (html! { <Badge /> }, "Badge"),
        (html! { <BadgeAlert /> }, "Badge Alert"),
        (html! { <BadgeCent /> }, "Badge Cent"),
        (html! { <BadgeCheck /> }, "Badge Check"),
        (html! { <BadgeDollarSign /> }, "Badge Dollar Sign"),
        (html! { <BadgeEuro /> }, "Badge Euro"),
        (html! { <BadgeHelp /> }, "Badge Help"),
        (html! { <BadgeIndianRupee /> }, "Badge Indian Rupee"),
        (html! { <BadgeInfo /> }, "Badge Info"),
        (html! { <BadgeJapaneseYen /> }, "Badge Japanese Yen"),
        (html! { <BadgeMinus /> }, "Badge Minus"),
        (html! { <BadgePercent /> }, "Badge Percent"),
        (html! { <BadgePlus /> }, "Badge Plus"),
        (html! { <BadgePoundSterling /> }, "Badge Pound Sterling"),
        (html! { <BadgeRussianRuble /> }, "Badge Russian Ruble"),
        (html! { <BadgeSwissFranc /> }, "Badge Swiss Franc"),
        (html! { <BadgeX /> }, "Badge X"),
        (html! { <BaggageClaim /> }, "Baggage Claim"),
        (html! { <Ban /> }, "Ban"),
        (html! { <Banana /> }, "Banana"),
        (html! { <Bandage /> }, "Bandage"),
        (html! { <Banknote /> }, "Banknote"),
        (html! { <BanknoteArrowDown /> }, "Banknote Arrow Down"),
        (html! { <BanknoteArrowUp /> }, "Banknote Arrow Up"),
        (html! { <BanknoteX /> }, "Banknote X"),
        (html! { <Barcode /> }, "Barcode"),
        (html! { <Baseline /> }, "Baseline"),
        (html! { <Bath /> }, "Bath"),
        (html! { <Battery /> }, "Battery"),
        (html! { <BatteryCharging /> }, "Battery Charging"),
        (html! { <BatteryFull /> }, "Battery Full"),
        (html! { <BatteryLow /> }, "Battery Low"),
        (html! { <BatteryMedium /> }, "Battery Medium"),
        (html! { <BatteryPlus /> }, "Battery Plus"),
        (html! { <BatteryWarning /> }, "Battery Warning"),
        (html! { <Beaker /> }, "Beaker"),
        (html! { <Bean /> }, "Bean"),
        (html! { <BeanOff /> }, "Bean Off"),
        (html! { <Bed /> }, "Bed"),
        (html! { <BedDouble /> }, "Bed Double"),
        (html! { <BedSingle /> }, "Bed Single"),
        (html! { <Beef /> }, "Beef"),
        (html! { <Beer /> }, "Beer"),
        (html! { <BeerOff /> }, "Beer Off"),
        (html! { <Bell /> }, "Bell"),
        (html! { <BellDot /> }, "Bell Dot"),
        (html! { <BellElectric /> }, "Bell Electric"),
        (html! { <BellMinus /> }, "Bell Minus"),
        (html! { <BellOff /> }, "Bell Off"),
        (html! { <BellPlus /> }, "Bell Plus"),
        (html! { <BellRing /> }, "Bell Ring"),
        (html! { <BetweenHorizontalEnd /> }, "Between Horizontal End"),
        (
            html! { <BetweenHorizontalStart /> },
            "Between Horizontal Start",
        ),
        (html! { <BetweenVerticalEnd /> }, "Between Vertical End"),
        (html! { <BetweenVerticalStart /> }, "Between Vertical Start"),
        (html! { <BicepsFlexed /> }, "Biceps Flexed"),
        (html! { <Bike /> }, "Bike"),
        (html! { <Binary /> }, "Binary"),
        (html! { <Binoculars /> }, "Binoculars"),
        (html! { <Biohazard /> }, "Biohazard"),
        (html! { <Bird /> }, "Bird"),
        (html! { <Bitcoin /> }, "Bitcoin"),
        (html! { <Blend /> }, "Blend"),
        (html! { <Blinds /> }, "Blinds"),
        (html! { <Blocks /> }, "Blocks"),
        (html! { <Bluetooth /> }, "Bluetooth"),
        (html! { <BluetoothConnected /> }, "Bluetooth Connected"),
        (html! { <BluetoothOff /> }, "Bluetooth Off"),
        (html! { <BluetoothSearching /> }, "Bluetooth Searching"),
        (html! { <Bold /> }, "Bold"),
        (html! { <Bolt /> }, "Bolt"),
        (html! { <Bomb /> }, "Bomb"),
        (html! { <Bone /> }, "Bone"),
        (html! { <Book /> }, "Book"),
        (html! { <BookA /> }, "Book A"),
        (html! { <BookAudio /> }, "Book Audio"),
        (html! { <BookCheck /> }, "Book Check"),
        (html! { <BookCopy /> }, "Book Copy"),
        (html! { <BookDashed /> }, "Book Dashed"),
        (html! { <BookDown /> }, "Book Down"),
        (html! { <BookHeadphones /> }, "Book Headphones"),
        (html! { <BookHeart /> }, "Book Heart"),
        (html! { <BookImage /> }, "Book Image"),
        (html! { <BookKey /> }, "Book Key"),
        (html! { <BookLock /> }, "Book Lock"),
        (html! { <BookMarked /> }, "Book Marked"),
        (html! { <BookMinus /> }, "Book Minus"),
        (html! { <BookOpen /> }, "Book Open"),
        (html! { <BookOpenCheck /> }, "Book Open Check"),
        (html! { <BookOpenText /> }, "Book Open Text"),
        (html! { <BookPlus /> }, "Book Plus"),
        (html! { <BookText /> }, "Book Text"),
        (html! { <BookType /> }, "Book Type"),
        (html! { <BookUp /> }, "Book Up"),
        (html! { <BookUp2 /> }, "Book Up 2"),
        (html! { <BookUser /> }, "Book User"),
        (html! { <BookX /> }, "Book X"),
        (html! { <Bookmark /> }, "Bookmark"),
        (html! { <BookmarkCheck /> }, "Bookmark Check"),
        (html! { <BookmarkMinus /> }, "Bookmark Minus"),
        (html! { <BookmarkPlus /> }, "Bookmark Plus"),
        (html! { <BookmarkX /> }, "Bookmark X"),
        (html! { <BoomBox /> }, "Boom Box"),
        (html! { <Bot /> }, "Bot"),
        (html! { <BotMessageSquare /> }, "Bot Message Square"),
        (html! { <BotOff /> }, "Bot Off"),
        (html! { <BowArrow /> }, "Bow Arrow"),
        (html! { <Box /> }, "Box"),
        (html! { <Boxes /> }, "Boxes"),
        (html! { <Braces /> }, "Braces"),
        (html! { <Brackets /> }, "Brackets"),
        (html! { <Brain /> }, "Brain"),
        (html! { <BrainCircuit /> }, "Brain Circuit"),
        (html! { <BrainCog /> }, "Brain Cog"),
        (html! { <BrickWall /> }, "Brick Wall"),
        (html! { <BrickWallFire /> }, "Brick Wall Fire"),
        (html! { <Briefcase /> }, "Briefcase"),
        (html! { <BriefcaseBusiness /> }, "Briefcase Business"),
        (
            html! { <BriefcaseConveyorBelt /> },
            "Briefcase Conveyor Belt",
        ),
        (html! { <BriefcaseMedical /> }, "Briefcase Medical"),
        (html! { <BringToFront /> }, "Bring To Front"),
        (html! { <Brush /> }, "Brush"),
        (html! { <BrushCleaning /> }, "Brush Cleaning"),
        (html! { <Bubbles /> }, "Bubbles"),
        (html! { <Bug /> }, "Bug"),
        (html! { <BugOff /> }, "Bug Off"),
        (html! { <BugPlay /> }, "Bug Play"),
        (html! { <Building /> }, "Building"),
        (html! { <Building2 /> }, "Building 2"),
        (html! { <Bus /> }, "Bus"),
        (html! { <BusFront /> }, "Bus Front"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsC() -> Html {
    let icons = [
        (html! { <Cable /> }, "Cable"),
        (html! { <CableCar /> }, "Cable Car"),
        (html! { <Cake /> }, "Cake"),
        (html! { <CakeSlice /> }, "Cake Slice"),
        (html! { <Calculator /> }, "Calculator"),
        (html! { <Calendar /> }, "Calendar"),
        (html! { <Calendar1 /> }, "Calendar 1"),
        (html! { <CalendarArrowDown /> }, "Calendar Arrow Down"),
        (html! { <CalendarArrowUp /> }, "Calendar Arrow Up"),
        (html! { <CalendarCheck /> }, "Calendar Check"),
        (html! { <CalendarCheck2 /> }, "Calendar Check 2"),
        (html! { <CalendarClock /> }, "Calendar Clock"),
        (html! { <CalendarCog /> }, "Calendar Cog"),
        (html! { <CalendarDays /> }, "Calendar Days"),
        (html! { <CalendarFold /> }, "Calendar Fold"),
        (html! { <CalendarHeart /> }, "Calendar Heart"),
        (html! { <CalendarMinus /> }, "Calendar Minus"),
        (html! { <CalendarMinus2 /> }, "Calendar Minus 2"),
        (html! { <CalendarOff /> }, "Calendar Off"),
        (html! { <CalendarPlus /> }, "Calendar Plus"),
        (html! { <CalendarPlus2 /> }, "Calendar Plus 2"),
        (html! { <CalendarRange /> }, "Calendar Range"),
        (html! { <CalendarSearch /> }, "Calendar Search"),
        (html! { <CalendarSync /> }, "Calendar Sync"),
        (html! { <CalendarX /> }, "Calendar X"),
        (html! { <CalendarX2 /> }, "Calendar X 2"),
        (html! { <Camera /> }, "Camera"),
        (html! { <CameraOff /> }, "Camera Off"),
        (html! { <Candy /> }, "Candy"),
        (html! { <CandyCane /> }, "Candy Cane"),
        (html! { <CandyOff /> }, "Candy Off"),
        (html! { <Cannabis /> }, "Cannabis"),
        (html! { <Captions /> }, "Captions"),
        (html! { <CaptionsOff /> }, "Captions Off"),
        (html! { <Car /> }, "Car"),
        (html! { <CarFront /> }, "Car Front"),
        (html! { <CarTaxiFront /> }, "Car Taxi Front"),
        (html! { <Caravan /> }, "Caravan"),
        (html! { <Carrot /> }, "Carrot"),
        (html! { <CaseLower /> }, "Case Lower"),
        (html! { <CaseSensitive /> }, "Case Sensitive"),
        (html! { <CaseUpper /> }, "Case Upper"),
        (html! { <CassetteTape /> }, "Cassette Tape"),
        (html! { <Cast /> }, "Cast"),
        (html! { <Castle /> }, "Castle"),
        (html! { <Cat /> }, "Cat"),
        (html! { <Cctv /> }, "Cctv"),
        (html! { <ChartArea /> }, "Chart Area"),
        (html! { <ChartBar /> }, "Chart Bar"),
        (html! { <ChartBarBig /> }, "Chart Bar Big"),
        (html! { <ChartBarDecreasing /> }, "Chart Bar Decreasing"),
        (html! { <ChartBarIncreasing /> }, "Chart Bar Increasing"),
        (html! { <ChartBarStacked /> }, "Chart Bar Stacked"),
        (html! { <ChartCandlestick /> }, "Chart Candlestick"),
        (html! { <ChartColumn /> }, "Chart Column"),
        (html! { <ChartColumnBig /> }, "Chart Column Big"),
        (
            html! { <ChartColumnDecreasing /> },
            "Chart Column Decreasing",
        ),
        (
            html! { <ChartColumnIncreasing /> },
            "Chart Column Increasing",
        ),
        (html! { <ChartColumnStacked /> }, "Chart Column Stacked"),
        (html! { <ChartGantt /> }, "Chart Gantt"),
        (html! { <ChartLine /> }, "Chart Line"),
        (html! { <ChartNetwork /> }, "Chart Network"),
        (html! { <ChartNoAxesColumn /> }, "Chart No Axes Column"),
        (
            html! { <ChartNoAxesColumnDecreasing /> },
            "Chart No Axes Column Decreasing",
        ),
        (
            html! { <ChartNoAxesColumnIncreasing /> },
            "Chart No Axes Column Increasing",
        ),
        (html! { <ChartNoAxesCombined /> }, "Chart No Axes Combined"),
        (html! { <ChartNoAxesGantt /> }, "Chart No Axes Gantt"),
        (html! { <ChartPie /> }, "Chart Pie"),
        (html! { <ChartScatter /> }, "Chart Scatter"),
        (html! { <ChartSpline /> }, "Chart Spline"),
        (html! { <Check /> }, "Check"),
        (html! { <CheckCheck /> }, "Check Check"),
        (html! { <CheckLine /> }, "Check Line"),
        (html! { <ChefHat /> }, "Chef Hat"),
        (html! { <Cherry /> }, "Cherry"),
        (html! { <ChevronDown /> }, "Chevron Down"),
        (html! { <ChevronFirst /> }, "Chevron First"),
        (html! { <ChevronLast /> }, "Chevron Last"),
        (html! { <ChevronLeft /> }, "Chevron Left"),
        (html! { <ChevronRight /> }, "Chevron Right"),
        (html! { <ChevronUp /> }, "Chevron Up"),
        (html! { <ChevronsDown /> }, "Chevrons Down"),
        (html! { <ChevronsDownUp /> }, "Chevrons Down Up"),
        (html! { <ChevronsLeft /> }, "Chevrons Left"),
        (html! { <ChevronsLeftRight /> }, "Chevrons Left Right"),
        (
            html! { <ChevronsLeftRightEllipsis /> },
            "Chevrons Left Right Ellipsis",
        ),
        (html! { <ChevronsRight /> }, "Chevrons Right"),
        (html! { <ChevronsRightLeft /> }, "Chevrons Right Left"),
        (html! { <ChevronsUp /> }, "Chevrons Up"),
        (html! { <ChevronsUpDown /> }, "Chevrons Up Down"),
        (html! { <Chrome /> }, "Chrome"),
        (html! { <Church /> }, "Church"),
        (html! { <Cigarette /> }, "Cigarette"),
        (html! { <CigaretteOff /> }, "Cigarette Off"),
        (html! { <Circle /> }, "Circle"),
        (html! { <CircleAlert /> }, "Circle Alert"),
        (html! { <CircleArrowDown /> }, "Circle Arrow Down"),
        (html! { <CircleArrowLeft /> }, "Circle Arrow Left"),
        (
            html! { <CircleArrowOutDownLeft /> },
            "Circle Arrow Out Down Left",
        ),
        (
            html! { <CircleArrowOutDownRight /> },
            "Circle Arrow Out Down Right",
        ),
        (
            html! { <CircleArrowOutUpLeft /> },
            "Circle Arrow Out Up Left",
        ),
        (
            html! { <CircleArrowOutUpRight /> },
            "Circle Arrow Out Up Right",
        ),
        (html! { <CircleArrowRight /> }, "Circle Arrow Right"),
        (html! { <CircleArrowUp /> }, "Circle Arrow Up"),
        (html! { <CircleCheck /> }, "Circle Check"),
        (html! { <CircleCheckBig /> }, "Circle Check Big"),
        (html! { <CircleChevronDown /> }, "Circle Chevron Down"),
        (html! { <CircleChevronLeft /> }, "Circle Chevron Left"),
        (html! { <CircleChevronRight /> }, "Circle Chevron Right"),
        (html! { <CircleChevronUp /> }, "Circle Chevron Up"),
        (html! { <CircleDashed /> }, "Circle Dashed"),
        (html! { <CircleDivide /> }, "Circle Divide"),
        (html! { <CircleDollarSign /> }, "Circle Dollar Sign"),
        (html! { <CircleDot /> }, "Circle Dot"),
        (html! { <CircleDotDashed /> }, "Circle Dot Dashed"),
        (html! { <CircleEllipsis /> }, "Circle Ellipsis"),
        (html! { <CircleEqual /> }, "Circle Equal"),
        (html! { <CircleFadingArrowUp /> }, "Circle Fading Arrow Up"),
        (html! { <CircleFadingPlus /> }, "Circle Fading Plus"),
        (html! { <CircleGauge /> }, "Circle Gauge"),
        (html! { <CircleHelp /> }, "Circle Help"),
        (html! { <CircleMinus /> }, "Circle Minus"),
        (html! { <CircleOff /> }, "Circle Off"),
        (html! { <CircleParking /> }, "Circle Parking"),
        (html! { <CircleParkingOff /> }, "Circle Parking Off"),
        (html! { <CirclePause /> }, "Circle Pause"),
        (html! { <CirclePercent /> }, "Circle Percent"),
        (html! { <CirclePlay /> }, "Circle Play"),
        (html! { <CirclePlus /> }, "Circle Plus"),
        (html! { <CirclePower /> }, "Circle Power"),
        (html! { <CircleSlash /> }, "Circle Slash"),
        (html! { <CircleSlash2 /> }, "Circle Slash 2"),
        (html! { <CircleSmall /> }, "Circle Small"),
        (html! { <CircleStop /> }, "Circle Stop"),
        (html! { <CircleUser /> }, "Circle User"),
        (html! { <CircleUserRound /> }, "Circle User Round"),
        (html! { <CircleX /> }, "Circle X"),
        (html! { <CircuitBoard /> }, "Circuit Board"),
        (html! { <Citrus /> }, "Citrus"),
        (html! { <Clapperboard /> }, "Clapperboard"),
        (html! { <Clipboard /> }, "Clipboard"),
        (html! { <ClipboardCheck /> }, "Clipboard Check"),
        (html! { <ClipboardCopy /> }, "Clipboard Copy"),
        (html! { <ClipboardList /> }, "Clipboard List"),
        (html! { <ClipboardMinus /> }, "Clipboard Minus"),
        (html! { <ClipboardPaste /> }, "Clipboard Paste"),
        (html! { <ClipboardPen /> }, "Clipboard Pen"),
        (html! { <ClipboardPenLine /> }, "Clipboard Pen Line"),
        (html! { <ClipboardPlus /> }, "Clipboard Plus"),
        (html! { <ClipboardType /> }, "Clipboard Type"),
        (html! { <ClipboardX /> }, "Clipboard X"),
        (html! { <Clock /> }, "Clock"),
        (html! { <Clock1 /> }, "Clock 1"),
        (html! { <Clock10 /> }, "Clock 10"),
        (html! { <Clock11 /> }, "Clock 11"),
        (html! { <Clock12 /> }, "Clock 12"),
        (html! { <Clock2 /> }, "Clock 2"),
        (html! { <Clock3 /> }, "Clock 3"),
        (html! { <Clock4 /> }, "Clock 4"),
        (html! { <Clock5 /> }, "Clock 5"),
        (html! { <Clock6 /> }, "Clock 6"),
        (html! { <Clock7 /> }, "Clock 7"),
        (html! { <Clock8 /> }, "Clock 8"),
        (html! { <Clock9 /> }, "Clock 9"),
        (html! { <ClockAlert /> }, "Clock Alert"),
        (html! { <ClockArrowDown /> }, "Clock Arrow Down"),
        (html! { <ClockArrowUp /> }, "Clock Arrow Up"),
        (html! { <ClockFading /> }, "Clock Fading"),
        (html! { <ClockPlus /> }, "Clock Plus"),
        (html! { <Cloud /> }, "Cloud"),
        (html! { <CloudAlert /> }, "Cloud Alert"),
        (html! { <CloudCog /> }, "Cloud Cog"),
        (html! { <CloudDownload /> }, "Cloud Download"),
        (html! { <CloudDrizzle /> }, "Cloud Drizzle"),
        (html! { <CloudFog /> }, "Cloud Fog"),
        (html! { <CloudHail /> }, "Cloud Hail"),
        (html! { <CloudLightning /> }, "Cloud Lightning"),
        (html! { <CloudMoon /> }, "Cloud Moon"),
        (html! { <CloudMoonRain /> }, "Cloud Moon Rain"),
        (html! { <CloudOff /> }, "Cloud Off"),
        (html! { <CloudRain /> }, "Cloud Rain"),
        (html! { <CloudRainWind /> }, "Cloud Rain Wind"),
        (html! { <CloudSnow /> }, "Cloud Snow"),
        (html! { <CloudSun /> }, "Cloud Sun"),
        (html! { <CloudSunRain /> }, "Cloud Sun Rain"),
        (html! { <CloudUpload /> }, "Cloud Upload"),
        (html! { <Cloudy /> }, "Cloudy"),
        (html! { <Clover /> }, "Clover"),
        (html! { <Club /> }, "Club"),
        (html! { <Code /> }, "Code"),
        (html! { <CodeXml /> }, "Code Xml"),
        (html! { <Codepen /> }, "Codepen"),
        (html! { <Codesandbox /> }, "Codesandbox"),
        (html! { <Coffee /> }, "Coffee"),
        (html! { <Cog /> }, "Cog"),
        (html! { <Coins /> }, "Coins"),
        (html! { <Columns2 /> }, "Columns 2"),
        (html! { <Columns3 /> }, "Columns 3"),
        (html! { <Columns3Cog /> }, "Columns 3 Cog"),
        (html! { <Columns4 /> }, "Columns 4"),
        (html! { <Combine /> }, "Combine"),
        (html! { <Command /> }, "Command"),
        (html! { <Compass /> }, "Compass"),
        (html! { <Component /> }, "Component"),
        (html! { <Computer /> }, "Computer"),
        (html! { <ConciergeBell /> }, "Concierge Bell"),
        (html! { <Cone /> }, "Cone"),
        (html! { <Construction /> }, "Construction"),
        (html! { <Contact /> }, "Contact"),
        (html! { <ContactRound /> }, "Contact Round"),
        (html! { <Container /> }, "Container"),
        (html! { <Contrast /> }, "Contrast"),
        (html! { <Cookie /> }, "Cookie"),
        (html! { <CookingPot /> }, "Cooking Pot"),
        (html! { <Copy /> }, "Copy"),
        (html! { <CopyCheck /> }, "Copy Check"),
        (html! { <CopyMinus /> }, "Copy Minus"),
        (html! { <CopyPlus /> }, "Copy Plus"),
        (html! { <CopySlash /> }, "Copy Slash"),
        (html! { <CopyX /> }, "Copy X"),
        (html! { <Copyleft /> }, "Copyleft"),
        (html! { <Copyright /> }, "Copyright"),
        (html! { <CornerDownLeft /> }, "Corner Down Left"),
        (html! { <CornerDownRight /> }, "Corner Down Right"),
        (html! { <CornerLeftDown /> }, "Corner Left Down"),
        (html! { <CornerLeftUp /> }, "Corner Left Up"),
        (html! { <CornerRightDown /> }, "Corner Right Down"),
        (html! { <CornerRightUp /> }, "Corner Right Up"),
        (html! { <CornerUpLeft /> }, "Corner Up Left"),
        (html! { <CornerUpRight /> }, "Corner Up Right"),
        (html! { <Cpu /> }, "Cpu"),
        (html! { <CreativeCommons /> }, "Creative Commons"),
        (html! { <CreditCard /> }, "Credit Card"),
        (html! { <Croissant /> }, "Croissant"),
        (html! { <Crop /> }, "Crop"),
        (html! { <Cross /> }, "Cross"),
        (html! { <Crosshair /> }, "Crosshair"),
        (html! { <Crown /> }, "Crown"),
        (html! { <Cuboid /> }, "Cuboid"),
        (html! { <CupSoda /> }, "Cup Soda"),
        (html! { <Currency /> }, "Currency"),
        (html! { <Cylinder /> }, "Cylinder"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsD() -> Html {
    let icons = [
        (html! { <Dam /> }, "Dam"),
        (html! { <Database /> }, "Database"),
        (html! { <DatabaseBackup /> }, "Database Backup"),
        (html! { <DatabaseZap /> }, "Database Zap"),
        (html! { <DecimalsArrowLeft /> }, "Decimals Arrow Left"),
        (html! { <DecimalsArrowRight /> }, "Decimals Arrow Right"),
        (html! { <Delete /> }, "Delete"),
        (html! { <Dessert /> }, "Dessert"),
        (html! { <Diameter /> }, "Diameter"),
        (html! { <Diamond /> }, "Diamond"),
        (html! { <DiamondMinus /> }, "Diamond Minus"),
        (html! { <DiamondPercent /> }, "Diamond Percent"),
        (html! { <DiamondPlus /> }, "Diamond Plus"),
        (html! { <Dice1 /> }, "Dice 1"),
        (html! { <Dice2 /> }, "Dice 2"),
        (html! { <Dice3 /> }, "Dice 3"),
        (html! { <Dice4 /> }, "Dice 4"),
        (html! { <Dice5 /> }, "Dice 5"),
        (html! { <Dice6 /> }, "Dice 6"),
        (html! { <Dices /> }, "Dices"),
        (html! { <Diff /> }, "Diff"),
        (html! { <Disc /> }, "Disc"),
        (html! { <Disc2 /> }, "Disc 2"),
        (html! { <Disc3 /> }, "Disc 3"),
        (html! { <DiscAlbum /> }, "Disc Album"),
        (html! { <Divide /> }, "Divide"),
        (html! { <Dna /> }, "Dna"),
        (html! { <DnaOff /> }, "Dna Off"),
        (html! { <Dock /> }, "Dock"),
        (html! { <Dog /> }, "Dog"),
        (html! { <DollarSign /> }, "Dollar Sign"),
        (html! { <Donut /> }, "Donut"),
        (html! { <DoorClosed /> }, "Door Closed"),
        (html! { <DoorClosedLocked /> }, "Door Closed Locked"),
        (html! { <DoorOpen /> }, "Door Open"),
        (html! { <Dot /> }, "Dot"),
        (html! { <Download /> }, "Download"),
        (html! { <DraftingCompass /> }, "Drafting Compass"),
        (html! { <Drama /> }, "Drama"),
        (html! { <Dribbble /> }, "Dribbble"),
        (html! { <Drill /> }, "Drill"),
        (html! { <Droplet /> }, "Droplet"),
        (html! { <DropletOff /> }, "Droplet Off"),
        (html! { <Droplets /> }, "Droplets"),
        (html! { <Drum /> }, "Drum"),
        (html! { <Drumstick /> }, "Drumstick"),
        (html! { <Dumbbell /> }, "Dumbbell"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsE() -> Html {
    let icons = [
        (html! { <Ear /> }, "Ear"),
        (html! { <EarOff /> }, "Ear Off"),
        (html! { <Earth /> }, "Earth"),
        (html! { <EarthLock /> }, "Earth Lock"),
        (html! { <Eclipse /> }, "Eclipse"),
        (html! { <Egg /> }, "Egg"),
        (html! { <EggFried /> }, "Egg Fried"),
        (html! { <EggOff /> }, "Egg Off"),
        (html! { <Ellipsis /> }, "Ellipsis"),
        (html! { <EllipsisVertical /> }, "Ellipsis Vertical"),
        (html! { <Equal /> }, "Equal"),
        (html! { <EqualApproximately /> }, "Equal Approximately"),
        (html! { <EqualNot /> }, "Equal Not"),
        (html! { <Eraser /> }, "Eraser"),
        (html! { <EthernetPort /> }, "Ethernet Port"),
        (html! { <Euro /> }, "Euro"),
        (html! { <Expand /> }, "Expand"),
        (html! { <ExternalLink /> }, "External Link"),
        (html! { <Eye /> }, "Eye"),
        (html! { <EyeClosed /> }, "Eye Closed"),
        (html! { <EyeOff /> }, "Eye Off"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsF() -> Html {
    let icons = [
        (html! { <Facebook /> }, "Facebook"),
        (html! { <Factory /> }, "Factory"),
        (html! { <Fan /> }, "Fan"),
        (html! { <FastForward /> }, "Fast Forward"),
        (html! { <Feather /> }, "Feather"),
        (html! { <Fence /> }, "Fence"),
        (html! { <FerrisWheel /> }, "Ferris Wheel"),
        (html! { <Figma /> }, "Figma"),
        (html! { <File /> }, "File"),
        (html! { <FileArchive /> }, "File Archive"),
        (html! { <FileAudio /> }, "File Audio"),
        (html! { <FileAudio2 /> }, "File Audio 2"),
        (html! { <FileAxis3D /> }, "File Axis 3 D"),
        (html! { <FileBadge /> }, "File Badge"),
        (html! { <FileBadge2 /> }, "File Badge 2"),
        (html! { <FileBox /> }, "File Box"),
        (html! { <FileChartColumn /> }, "File Chart Column"),
        (
            html! { <FileChartColumnIncreasing /> },
            "File Chart Column Increasing",
        ),
        (html! { <FileChartLine /> }, "File Chart Line"),
        (html! { <FileChartPie /> }, "File Chart Pie"),
        (html! { <FileCheck /> }, "File Check"),
        (html! { <FileCheck2 /> }, "File Check 2"),
        (html! { <FileClock /> }, "File Clock"),
        (html! { <FileCode /> }, "File Code"),
        (html! { <FileCode2 /> }, "File Code 2"),
        (html! { <FileCog /> }, "File Cog"),
        (html! { <FileDiff /> }, "File Diff"),
        (html! { <FileDigit /> }, "File Digit"),
        (html! { <FileDown /> }, "File Down"),
        (html! { <FileHeart /> }, "File Heart"),
        (html! { <FileImage /> }, "File Image"),
        (html! { <FileInput /> }, "File Input"),
        (html! { <FileJson /> }, "File Json"),
        (html! { <FileJson2 /> }, "File Json 2"),
        (html! { <FileKey /> }, "File Key"),
        (html! { <FileKey2 /> }, "File Key 2"),
        (html! { <FileLock /> }, "File Lock"),
        (html! { <FileLock2 /> }, "File Lock 2"),
        (html! { <FileMinus /> }, "File Minus"),
        (html! { <FileMinus2 /> }, "File Minus 2"),
        (html! { <FileMusic /> }, "File Music"),
        (html! { <FileOutput /> }, "File Output"),
        (html! { <FilePen /> }, "File Pen"),
        (html! { <FilePenLine /> }, "File Pen Line"),
        (html! { <FilePlus /> }, "File Plus"),
        (html! { <FilePlus2 /> }, "File Plus 2"),
        (html! { <FileQuestion /> }, "File Question"),
        (html! { <FileScan /> }, "File Scan"),
        (html! { <FileSearch /> }, "File Search"),
        (html! { <FileSearch2 /> }, "File Search 2"),
        (html! { <FileSliders /> }, "File Sliders"),
        (html! { <FileSpreadsheet /> }, "File Spreadsheet"),
        (html! { <FileStack /> }, "File Stack"),
        (html! { <FileSymlink /> }, "File Symlink"),
        (html! { <FileTerminal /> }, "File Terminal"),
        (html! { <FileText /> }, "File Text"),
        (html! { <FileType /> }, "File Type"),
        (html! { <FileType2 /> }, "File Type 2"),
        (html! { <FileUp /> }, "File Up"),
        (html! { <FileUser /> }, "File User"),
        (html! { <FileVideo /> }, "File Video"),
        (html! { <FileVideo2 /> }, "File Video 2"),
        (html! { <FileVolume /> }, "File Volume"),
        (html! { <FileVolume2 /> }, "File Volume 2"),
        (html! { <FileWarning /> }, "File Warning"),
        (html! { <FileX /> }, "File X"),
        (html! { <FileX2 /> }, "File X 2"),
        (html! { <Files /> }, "Files"),
        (html! { <Film /> }, "Film"),
        (html! { <Fingerprint /> }, "Fingerprint"),
        (html! { <FireExtinguisher /> }, "Fire Extinguisher"),
        (html! { <Fish /> }, "Fish"),
        (html! { <FishOff /> }, "Fish Off"),
        (html! { <FishSymbol /> }, "Fish Symbol"),
        (html! { <Flag /> }, "Flag"),
        (html! { <FlagOff /> }, "Flag Off"),
        (html! { <FlagTriangleLeft /> }, "Flag Triangle Left"),
        (html! { <FlagTriangleRight /> }, "Flag Triangle Right"),
        (html! { <Flame /> }, "Flame"),
        (html! { <FlameKindling /> }, "Flame Kindling"),
        (html! { <Flashlight /> }, "Flashlight"),
        (html! { <FlashlightOff /> }, "Flashlight Off"),
        (html! { <FlaskConical /> }, "Flask Conical"),
        (html! { <FlaskConicalOff /> }, "Flask Conical Off"),
        (html! { <FlaskRound /> }, "Flask Round"),
        (html! { <FlipHorizontal /> }, "Flip Horizontal"),
        (html! { <FlipHorizontal2 /> }, "Flip Horizontal 2"),
        (html! { <FlipVertical /> }, "Flip Vertical"),
        (html! { <FlipVertical2 /> }, "Flip Vertical 2"),
        (html! { <Flower /> }, "Flower"),
        (html! { <Flower2 /> }, "Flower 2"),
        (html! { <Focus /> }, "Focus"),
        (html! { <FoldHorizontal /> }, "Fold Horizontal"),
        (html! { <FoldVertical /> }, "Fold Vertical"),
        (html! { <Folder /> }, "Folder"),
        (html! { <FolderArchive /> }, "Folder Archive"),
        (html! { <FolderCheck /> }, "Folder Check"),
        (html! { <FolderClock /> }, "Folder Clock"),
        (html! { <FolderClosed /> }, "Folder Closed"),
        (html! { <FolderCode /> }, "Folder Code"),
        (html! { <FolderCog /> }, "Folder Cog"),
        (html! { <FolderDot /> }, "Folder Dot"),
        (html! { <FolderDown /> }, "Folder Down"),
        (html! { <FolderGit /> }, "Folder Git"),
        (html! { <FolderGit2 /> }, "Folder Git 2"),
        (html! { <FolderHeart /> }, "Folder Heart"),
        (html! { <FolderInput /> }, "Folder Input"),
        (html! { <FolderKanban /> }, "Folder Kanban"),
        (html! { <FolderKey /> }, "Folder Key"),
        (html! { <FolderLock /> }, "Folder Lock"),
        (html! { <FolderMinus /> }, "Folder Minus"),
        (html! { <FolderOpen /> }, "Folder Open"),
        (html! { <FolderOpenDot /> }, "Folder Open Dot"),
        (html! { <FolderOutput /> }, "Folder Output"),
        (html! { <FolderPen /> }, "Folder Pen"),
        (html! { <FolderPlus /> }, "Folder Plus"),
        (html! { <FolderRoot /> }, "Folder Root"),
        (html! { <FolderSearch /> }, "Folder Search"),
        (html! { <FolderSearch2 /> }, "Folder Search 2"),
        (html! { <FolderSymlink /> }, "Folder Symlink"),
        (html! { <FolderSync /> }, "Folder Sync"),
        (html! { <FolderTree /> }, "Folder Tree"),
        (html! { <FolderUp /> }, "Folder Up"),
        (html! { <FolderX /> }, "Folder X"),
        (html! { <Folders /> }, "Folders"),
        (html! { <Footprints /> }, "Footprints"),
        (html! { <Forklift /> }, "Forklift"),
        (html! { <Forward /> }, "Forward"),
        (html! { <Frame /> }, "Frame"),
        (html! { <Framer /> }, "Framer"),
        (html! { <Frown /> }, "Frown"),
        (html! { <Fuel /> }, "Fuel"),
        (html! { <Fullscreen /> }, "Fullscreen"),
        (html! { <Funnel /> }, "Funnel"),
        (html! { <FunnelPlus /> }, "Funnel Plus"),
        (html! { <FunnelX /> }, "Funnel X"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsG() -> Html {
    let icons = [
        (html! { <GalleryHorizontal /> }, "Gallery Horizontal"),
        (html! { <GalleryHorizontalEnd /> }, "Gallery Horizontal End"),
        (html! { <GalleryThumbnails /> }, "Gallery Thumbnails"),
        (html! { <GalleryVertical /> }, "Gallery Vertical"),
        (html! { <GalleryVerticalEnd /> }, "Gallery Vertical End"),
        (html! { <Gamepad /> }, "Gamepad"),
        (html! { <Gamepad2 /> }, "Gamepad 2"),
        (html! { <Gauge /> }, "Gauge"),
        (html! { <Gavel /> }, "Gavel"),
        (html! { <Gem /> }, "Gem"),
        (html! { <Ghost /> }, "Ghost"),
        (html! { <Gift /> }, "Gift"),
        (html! { <GitBranch /> }, "Git Branch"),
        (html! { <GitBranchPlus /> }, "Git Branch Plus"),
        (html! { <GitCommitHorizontal /> }, "Git Commit Horizontal"),
        (html! { <GitCommitVertical /> }, "Git Commit Vertical"),
        (html! { <GitCompare /> }, "Git Compare"),
        (html! { <GitCompareArrows /> }, "Git Compare Arrows"),
        (html! { <GitFork /> }, "Git Fork"),
        (html! { <GitGraph /> }, "Git Graph"),
        (html! { <GitMerge /> }, "Git Merge"),
        (html! { <GitPullRequest /> }, "Git Pull Request"),
        (html! { <GitPullRequestArrow /> }, "Git Pull Request Arrow"),
        (
            html! { <GitPullRequestClosed /> },
            "Git Pull Request Closed",
        ),
        (
            html! { <GitPullRequestCreate /> },
            "Git Pull Request Create",
        ),
        (
            html! { <GitPullRequestCreateArrow /> },
            "Git Pull Request Create Arrow",
        ),
        (html! { <GitPullRequestDraft /> }, "Git Pull Request Draft"),
        (html! { <Github /> }, "Github"),
        (html! { <Gitlab /> }, "Gitlab"),
        (html! { <GlassWater /> }, "Glass Water"),
        (html! { <Glasses /> }, "Glasses"),
        (html! { <Globe /> }, "Globe"),
        (html! { <GlobeLock /> }, "Globe Lock"),
        (html! { <Goal /> }, "Goal"),
        (html! { <Gpu /> }, "Gpu"),
        (html! { <Grab /> }, "Grab"),
        (html! { <GraduationCap /> }, "Graduation Cap"),
        (html! { <Grape /> }, "Grape"),
        (html! { <Grid2X2 /> }, "Grid 2 X 2"),
        (html! { <Grid2X2Check /> }, "Grid 2 X 2 Check"),
        (html! { <Grid2X2Plus /> }, "Grid 2 X 2 Plus"),
        (html! { <Grid2X2X /> }, "Grid 2 X 2 X"),
        (html! { <Grid3X3 /> }, "Grid 3 X 3"),
        (html! { <Grip /> }, "Grip"),
        (html! { <GripHorizontal /> }, "Grip Horizontal"),
        (html! { <GripVertical /> }, "Grip Vertical"),
        (html! { <Group /> }, "Group"),
        (html! { <Guitar /> }, "Guitar"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsH() -> Html {
    let icons = [
        (html! { <Ham /> }, "Ham"),
        (html! { <Hamburger /> }, "Hamburger"),
        (html! { <Hammer /> }, "Hammer"),
        (html! { <Hand /> }, "Hand"),
        (html! { <HandCoins /> }, "Hand Coins"),
        (html! { <HandHeart /> }, "Hand Heart"),
        (html! { <HandHelping /> }, "Hand Helping"),
        (html! { <HandMetal /> }, "Hand Metal"),
        (html! { <HandPlatter /> }, "Hand Platter"),
        (html! { <Handshake /> }, "Handshake"),
        (html! { <HardDrive /> }, "Hard Drive"),
        (html! { <HardDriveDownload /> }, "Hard Drive Download"),
        (html! { <HardDriveUpload /> }, "Hard Drive Upload"),
        (html! { <HardHat /> }, "Hard Hat"),
        (html! { <Hash /> }, "Hash"),
        (html! { <Haze /> }, "Haze"),
        (html! { <HdmiPort /> }, "Hdmi Port"),
        (html! { <Heading /> }, "Heading"),
        (html! { <Heading1 /> }, "Heading 1"),
        (html! { <Heading2 /> }, "Heading 2"),
        (html! { <Heading3 /> }, "Heading 3"),
        (html! { <Heading4 /> }, "Heading 4"),
        (html! { <Heading5 /> }, "Heading 5"),
        (html! { <Heading6 /> }, "Heading 6"),
        (html! { <HeadphoneOff /> }, "Headphone Off"),
        (html! { <Headphones /> }, "Headphones"),
        (html! { <Headset /> }, "Headset"),
        (html! { <Heart /> }, "Heart"),
        (html! { <HeartCrack /> }, "Heart Crack"),
        (html! { <HeartHandshake /> }, "Heart Handshake"),
        (html! { <HeartMinus /> }, "Heart Minus"),
        (html! { <HeartOff /> }, "Heart Off"),
        (html! { <HeartPlus /> }, "Heart Plus"),
        (html! { <HeartPulse /> }, "Heart Pulse"),
        (html! { <Heater /> }, "Heater"),
        (html! { <Hexagon /> }, "Hexagon"),
        (html! { <Highlighter /> }, "Highlighter"),
        (html! { <History /> }, "History"),
        (html! { <Hop /> }, "Hop"),
        (html! { <HopOff /> }, "Hop Off"),
        (html! { <Hospital /> }, "Hospital"),
        (html! { <Hotel /> }, "Hotel"),
        (html! { <Hourglass /> }, "Hourglass"),
        (html! { <House /> }, "House"),
        (html! { <HousePlug /> }, "House Plug"),
        (html! { <HousePlus /> }, "House Plus"),
        (html! { <HouseWifi /> }, "House Wifi"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsI() -> Html {
    let icons = [
        (html! { <IceCreamBowl /> }, "Ice Cream Bowl"),
        (html! { <IceCreamCone /> }, "Ice Cream Cone"),
        (html! { <IdCard /> }, "Id Card"),
        (html! { <Image /> }, "Image"),
        (html! { <ImageDown /> }, "Image Down"),
        (html! { <ImageMinus /> }, "Image Minus"),
        (html! { <ImageOff /> }, "Image Off"),
        (html! { <ImagePlay /> }, "Image Play"),
        (html! { <ImagePlus /> }, "Image Plus"),
        (html! { <ImageUp /> }, "Image Up"),
        (html! { <ImageUpscale /> }, "Image Upscale"),
        (html! { <Images /> }, "Images"),
        (html! { <Import /> }, "Import"),
        (html! { <Inbox /> }, "Inbox"),
        (html! { <IndentDecrease /> }, "Indent Decrease"),
        (html! { <IndentIncrease /> }, "Indent Increase"),
        (html! { <IndianRupee /> }, "Indian Rupee"),
        (html! { <Infinity /> }, "Infinity"),
        (html! { <Info /> }, "Info"),
        (html! { <InspectionPanel /> }, "Inspection Panel"),
        (html! { <Instagram /> }, "Instagram"),
        (html! { <Italic /> }, "Italic"),
        (html! { <IterationCcw /> }, "Iteration Ccw"),
        (html! { <IterationCw /> }, "Iteration Cw"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsJ() -> Html {
    let icons = [
        (html! { <JapaneseYen /> }, "Japanese Yen"),
        (html! { <Joystick /> }, "Joystick"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsK() -> Html {
    let icons = [
        (html! { <Kanban /> }, "Kanban"),
        (html! { <Key /> }, "Key"),
        (html! { <KeyRound /> }, "Key Round"),
        (html! { <KeySquare /> }, "Key Square"),
        (html! { <Keyboard /> }, "Keyboard"),
        (html! { <KeyboardMusic /> }, "Keyboard Music"),
        (html! { <KeyboardOff /> }, "Keyboard Off"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsL() -> Html {
    let icons = [
        (html! { <Lamp /> }, "Lamp"),
        (html! { <LampCeiling /> }, "Lamp Ceiling"),
        (html! { <LampDesk /> }, "Lamp Desk"),
        (html! { <LampFloor /> }, "Lamp Floor"),
        (html! { <LampWallDown /> }, "Lamp Wall Down"),
        (html! { <LampWallUp /> }, "Lamp Wall Up"),
        (html! { <LandPlot /> }, "Land Plot"),
        (html! { <Landmark /> }, "Landmark"),
        (html! { <Languages /> }, "Languages"),
        (html! { <Laptop /> }, "Laptop"),
        (html! { <LaptopMinimal /> }, "Laptop Minimal"),
        (html! { <LaptopMinimalCheck /> }, "Laptop Minimal Check"),
        (html! { <Lasso /> }, "Lasso"),
        (html! { <LassoSelect /> }, "Lasso Select"),
        (html! { <Laugh /> }, "Laugh"),
        (html! { <Layers /> }, "Layers"),
        (html! { <Layers2 /> }, "Layers 2"),
        (html! { <LayoutDashboard /> }, "Layout Dashboard"),
        (html! { <LayoutGrid /> }, "Layout Grid"),
        (html! { <LayoutList /> }, "Layout List"),
        (html! { <LayoutPanelLeft /> }, "Layout Panel Left"),
        (html! { <LayoutPanelTop /> }, "Layout Panel Top"),
        (html! { <LayoutTemplate /> }, "Layout Template"),
        (html! { <Leaf /> }, "Leaf"),
        (html! { <LeafyGreen /> }, "Leafy Green"),
        (html! { <Lectern /> }, "Lectern"),
        (html! { <LetterText /> }, "Letter Text"),
        (html! { <Library /> }, "Library"),
        (html! { <LibraryBig /> }, "Library Big"),
        (html! { <LifeBuoy /> }, "Life Buoy"),
        (html! { <Ligature /> }, "Ligature"),
        (html! { <Lightbulb /> }, "Lightbulb"),
        (html! { <LightbulbOff /> }, "Lightbulb Off"),
        (html! { <Link /> }, "Link"),
        (html! { <Link2 /> }, "Link 2"),
        (html! { <Link2Off /> }, "Link 2 Off"),
        (html! { <Linkedin /> }, "Linkedin"),
        (html! { <List /> }, "List"),
        (html! { <ListCheck /> }, "List Check"),
        (html! { <ListChecks /> }, "List Checks"),
        (html! { <ListCollapse /> }, "List Collapse"),
        (html! { <ListEnd /> }, "List End"),
        (html! { <ListFilter /> }, "List Filter"),
        (html! { <ListFilterPlus /> }, "List Filter Plus"),
        (html! { <ListMinus /> }, "List Minus"),
        (html! { <ListMusic /> }, "List Music"),
        (html! { <ListOrdered /> }, "List Ordered"),
        (html! { <ListPlus /> }, "List Plus"),
        (html! { <ListRestart /> }, "List Restart"),
        (html! { <ListStart /> }, "List Start"),
        (html! { <ListTodo /> }, "List Todo"),
        (html! { <ListTree /> }, "List Tree"),
        (html! { <ListVideo /> }, "List Video"),
        (html! { <ListX /> }, "List X"),
        (html! { <Loader /> }, "Loader"),
        (html! { <LoaderCircle /> }, "Loader Circle"),
        (html! { <LoaderPinwheel /> }, "Loader Pinwheel"),
        (html! { <Locate /> }, "Locate"),
        (html! { <LocateFixed /> }, "Locate Fixed"),
        (html! { <LocateOff /> }, "Locate Off"),
        (html! { <LocationEdit /> }, "Location Edit"),
        (html! { <Lock /> }, "Lock"),
        (html! { <LockKeyhole /> }, "Lock Keyhole"),
        (html! { <LockKeyholeOpen /> }, "Lock Keyhole Open"),
        (html! { <LockOpen /> }, "Lock Open"),
        (html! { <LogIn /> }, "Log In"),
        (html! { <LogOut /> }, "Log Out"),
        (html! { <Logs /> }, "Logs"),
        (html! { <Lollipop /> }, "Lollipop"),
        (html! { <Luggage /> }, "Luggage"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsM() -> Html {
    let icons = [
        (html! { <Magnet /> }, "Magnet"),
        (html! { <Mail /> }, "Mail"),
        (html! { <MailCheck /> }, "Mail Check"),
        (html! { <MailMinus /> }, "Mail Minus"),
        (html! { <MailOpen /> }, "Mail Open"),
        (html! { <MailPlus /> }, "Mail Plus"),
        (html! { <MailQuestion /> }, "Mail Question"),
        (html! { <MailSearch /> }, "Mail Search"),
        (html! { <MailWarning /> }, "Mail Warning"),
        (html! { <MailX /> }, "Mail X"),
        (html! { <Mailbox /> }, "Mailbox"),
        (html! { <Mails /> }, "Mails"),
        (html! { <Map /> }, "Map"),
        (html! { <MapPin /> }, "Map Pin"),
        (html! { <MapPinCheck /> }, "Map Pin Check"),
        (html! { <MapPinCheckInside /> }, "Map Pin Check Inside"),
        (html! { <MapPinHouse /> }, "Map Pin House"),
        (html! { <MapPinMinus /> }, "Map Pin Minus"),
        (html! { <MapPinMinusInside /> }, "Map Pin Minus Inside"),
        (html! { <MapPinOff /> }, "Map Pin Off"),
        (html! { <MapPinPlus /> }, "Map Pin Plus"),
        (html! { <MapPinPlusInside /> }, "Map Pin Plus Inside"),
        (html! { <MapPinX /> }, "Map Pin X"),
        (html! { <MapPinXInside /> }, "Map Pin X Inside"),
        (html! { <MapPinned /> }, "Map Pinned"),
        (html! { <MapPlus /> }, "Map Plus"),
        (html! { <Mars /> }, "Mars"),
        (html! { <MarsStroke /> }, "Mars Stroke"),
        (html! { <Martini /> }, "Martini"),
        (html! { <Maximize /> }, "Maximize"),
        (html! { <Maximize2 /> }, "Maximize 2"),
        (html! { <Medal /> }, "Medal"),
        (html! { <Megaphone /> }, "Megaphone"),
        (html! { <MegaphoneOff /> }, "Megaphone Off"),
        (html! { <Meh /> }, "Meh"),
        (html! { <MemoryStick /> }, "Memory Stick"),
        (html! { <Menu /> }, "Menu"),
        (html! { <Merge /> }, "Merge"),
        (html! { <MessageCircle /> }, "Message Circle"),
        (html! { <MessageCircleCode /> }, "Message Circle Code"),
        (html! { <MessageCircleDashed /> }, "Message Circle Dashed"),
        (html! { <MessageCircleHeart /> }, "Message Circle Heart"),
        (html! { <MessageCircleMore /> }, "Message Circle More"),
        (html! { <MessageCircleOff /> }, "Message Circle Off"),
        (html! { <MessageCirclePlus /> }, "Message Circle Plus"),
        (
            html! { <MessageCircleQuestion /> },
            "Message Circle Question",
        ),
        (html! { <MessageCircleReply /> }, "Message Circle Reply"),
        (html! { <MessageCircleWarning /> }, "Message Circle Warning"),
        (html! { <MessageCircleX /> }, "Message Circle X"),
        (html! { <MessageSquare /> }, "Message Square"),
        (html! { <MessageSquareCode /> }, "Message Square Code"),
        (html! { <MessageSquareDashed /> }, "Message Square Dashed"),
        (html! { <MessageSquareDiff /> }, "Message Square Diff"),
        (html! { <MessageSquareDot /> }, "Message Square Dot"),
        (html! { <MessageSquareHeart /> }, "Message Square Heart"),
        (html! { <MessageSquareLock /> }, "Message Square Lock"),
        (html! { <MessageSquareMore /> }, "Message Square More"),
        (html! { <MessageSquareOff /> }, "Message Square Off"),
        (html! { <MessageSquarePlus /> }, "Message Square Plus"),
        (html! { <MessageSquareQuote /> }, "Message Square Quote"),
        (html! { <MessageSquareReply /> }, "Message Square Reply"),
        (html! { <MessageSquareShare /> }, "Message Square Share"),
        (html! { <MessageSquareText /> }, "Message Square Text"),
        (html! { <MessageSquareWarning /> }, "Message Square Warning"),
        (html! { <MessageSquareX /> }, "Message Square X"),
        (html! { <MessagesSquare /> }, "Messages Square"),
        (html! { <Mic /> }, "Mic"),
        (html! { <MicOff /> }, "Mic Off"),
        (html! { <MicVocal /> }, "Mic Vocal"),
        (html! { <Microchip /> }, "Microchip"),
        (html! { <Microscope /> }, "Microscope"),
        (html! { <Microwave /> }, "Microwave"),
        (html! { <Milestone /> }, "Milestone"),
        (html! { <Milk /> }, "Milk"),
        (html! { <MilkOff /> }, "Milk Off"),
        (html! { <Minimize /> }, "Minimize"),
        (html! { <Minimize2 /> }, "Minimize 2"),
        (html! { <Minus /> }, "Minus"),
        (html! { <Monitor /> }, "Monitor"),
        (html! { <MonitorCheck /> }, "Monitor Check"),
        (html! { <MonitorCog /> }, "Monitor Cog"),
        (html! { <MonitorDot /> }, "Monitor Dot"),
        (html! { <MonitorDown /> }, "Monitor Down"),
        (html! { <MonitorOff /> }, "Monitor Off"),
        (html! { <MonitorPause /> }, "Monitor Pause"),
        (html! { <MonitorPlay /> }, "Monitor Play"),
        (html! { <MonitorSmartphone /> }, "Monitor Smartphone"),
        (html! { <MonitorSpeaker /> }, "Monitor Speaker"),
        (html! { <MonitorStop /> }, "Monitor Stop"),
        (html! { <MonitorUp /> }, "Monitor Up"),
        (html! { <MonitorX /> }, "Monitor X"),
        (html! { <Moon /> }, "Moon"),
        (html! { <MoonStar /> }, "Moon Star"),
        (html! { <Mountain /> }, "Mountain"),
        (html! { <MountainSnow /> }, "Mountain Snow"),
        (html! { <Mouse /> }, "Mouse"),
        (html! { <MouseOff /> }, "Mouse Off"),
        (html! { <MousePointer /> }, "Mouse Pointer"),
        (html! { <MousePointer2 /> }, "Mouse Pointer 2"),
        (html! { <MousePointerBan /> }, "Mouse Pointer Ban"),
        (html! { <MousePointerClick /> }, "Mouse Pointer Click"),
        (html! { <Move /> }, "Move"),
        (html! { <Move3D /> }, "Move 3 D"),
        (html! { <MoveDiagonal /> }, "Move Diagonal"),
        (html! { <MoveDiagonal2 /> }, "Move Diagonal 2"),
        (html! { <MoveDown /> }, "Move Down"),
        (html! { <MoveDownLeft /> }, "Move Down Left"),
        (html! { <MoveDownRight /> }, "Move Down Right"),
        (html! { <MoveHorizontal /> }, "Move Horizontal"),
        (html! { <MoveLeft /> }, "Move Left"),
        (html! { <MoveRight /> }, "Move Right"),
        (html! { <MoveUp /> }, "Move Up"),
        (html! { <MoveUpLeft /> }, "Move Up Left"),
        (html! { <MoveUpRight /> }, "Move Up Right"),
        (html! { <MoveVertical /> }, "Move Vertical"),
        (html! { <Music /> }, "Music"),
        (html! { <Music2 /> }, "Music 2"),
        (html! { <Music3 /> }, "Music 3"),
        (html! { <Music4 /> }, "Music 4"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsN() -> Html {
    let icons = [
        (html! { <Navigation /> }, "Navigation"),
        (html! { <Navigation2 /> }, "Navigation 2"),
        (html! { <Navigation2Off /> }, "Navigation 2 Off"),
        (html! { <NavigationOff /> }, "Navigation Off"),
        (html! { <Network /> }, "Network"),
        (html! { <Newspaper /> }, "Newspaper"),
        (html! { <Nfc /> }, "Nfc"),
        (html! { <NonBinary /> }, "Non Binary"),
        (html! { <Notebook /> }, "Notebook"),
        (html! { <NotebookPen /> }, "Notebook Pen"),
        (html! { <NotebookTabs /> }, "Notebook Tabs"),
        (html! { <NotebookText /> }, "Notebook Text"),
        (html! { <NotepadText /> }, "Notepad Text"),
        (html! { <NotepadTextDashed /> }, "Notepad Text Dashed"),
        (html! { <Nut /> }, "Nut"),
        (html! { <NutOff /> }, "Nut Off"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsO() -> Html {
    let icons = [
        (html! { <Octagon /> }, "Octagon"),
        (html! { <OctagonAlert /> }, "Octagon Alert"),
        (html! { <OctagonMinus /> }, "Octagon Minus"),
        (html! { <OctagonPause /> }, "Octagon Pause"),
        (html! { <OctagonX /> }, "Octagon X"),
        (html! { <Omega /> }, "Omega"),
        (html! { <Option /> }, "Option"),
        (html! { <Orbit /> }, "Orbit"),
        (html! { <Origami /> }, "Origami"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsP() -> Html {
    let icons = [
        (html! { <Package /> }, "Package"),
        (html! { <Package2 /> }, "Package 2"),
        (html! { <PackageCheck /> }, "Package Check"),
        (html! { <PackageMinus /> }, "Package Minus"),
        (html! { <PackageOpen /> }, "Package Open"),
        (html! { <PackagePlus /> }, "Package Plus"),
        (html! { <PackageSearch /> }, "Package Search"),
        (html! { <PackageX /> }, "Package X"),
        (html! { <PaintBucket /> }, "Paint Bucket"),
        (html! { <PaintRoller /> }, "Paint Roller"),
        (html! { <Paintbrush /> }, "Paintbrush"),
        (html! { <PaintbrushVertical /> }, "Paintbrush Vertical"),
        (html! { <Palette /> }, "Palette"),
        (html! { <Panda /> }, "Panda"),
        (html! { <PanelBottom /> }, "Panel Bottom"),
        (html! { <PanelBottomClose /> }, "Panel Bottom Close"),
        (html! { <PanelBottomDashed /> }, "Panel Bottom Dashed"),
        (html! { <PanelBottomOpen /> }, "Panel Bottom Open"),
        (html! { <PanelLeft /> }, "Panel Left"),
        (html! { <PanelLeftClose /> }, "Panel Left Close"),
        (html! { <PanelLeftDashed /> }, "Panel Left Dashed"),
        (html! { <PanelLeftOpen /> }, "Panel Left Open"),
        (html! { <PanelRight /> }, "Panel Right"),
        (html! { <PanelRightClose /> }, "Panel Right Close"),
        (html! { <PanelRightDashed /> }, "Panel Right Dashed"),
        (html! { <PanelRightOpen /> }, "Panel Right Open"),
        (html! { <PanelTop /> }, "Panel Top"),
        (html! { <PanelTopClose /> }, "Panel Top Close"),
        (html! { <PanelTopDashed /> }, "Panel Top Dashed"),
        (html! { <PanelTopOpen /> }, "Panel Top Open"),
        (html! { <PanelsLeftBottom /> }, "Panels Left Bottom"),
        (html! { <PanelsRightBottom /> }, "Panels Right Bottom"),
        (html! { <PanelsTopLeft /> }, "Panels Top Left"),
        (html! { <Paperclip /> }, "Paperclip"),
        (html! { <Parentheses /> }, "Parentheses"),
        (html! { <ParkingMeter /> }, "Parking Meter"),
        (html! { <PartyPopper /> }, "Party Popper"),
        (html! { <Pause /> }, "Pause"),
        (html! { <PawPrint /> }, "Paw Print"),
        (html! { <PcCase /> }, "Pc Case"),
        (html! { <Pen /> }, "Pen"),
        (html! { <PenLine /> }, "Pen Line"),
        (html! { <PenOff /> }, "Pen Off"),
        (html! { <PenTool /> }, "Pen Tool"),
        (html! { <Pencil /> }, "Pencil"),
        (html! { <PencilLine /> }, "Pencil Line"),
        (html! { <PencilOff /> }, "Pencil Off"),
        (html! { <PencilRuler /> }, "Pencil Ruler"),
        (html! { <Pentagon /> }, "Pentagon"),
        (html! { <Percent /> }, "Percent"),
        (html! { <PersonStanding /> }, "Person Standing"),
        (html! { <PhilippinePeso /> }, "Philippine Peso"),
        (html! { <Phone /> }, "Phone"),
        (html! { <PhoneCall /> }, "Phone Call"),
        (html! { <PhoneForwarded /> }, "Phone Forwarded"),
        (html! { <PhoneIncoming /> }, "Phone Incoming"),
        (html! { <PhoneMissed /> }, "Phone Missed"),
        (html! { <PhoneOff /> }, "Phone Off"),
        (html! { <PhoneOutgoing /> }, "Phone Outgoing"),
        (html! { <Pi /> }, "Pi"),
        (html! { <Piano /> }, "Piano"),
        (html! { <Pickaxe /> }, "Pickaxe"),
        (html! { <PictureInPicture /> }, "Picture In Picture"),
        (html! { <PictureInPicture2 /> }, "Picture In Picture 2"),
        (html! { <PiggyBank /> }, "Piggy Bank"),
        (html! { <Pilcrow /> }, "Pilcrow"),
        (html! { <PilcrowLeft /> }, "Pilcrow Left"),
        (html! { <PilcrowRight /> }, "Pilcrow Right"),
        (html! { <Pill /> }, "Pill"),
        (html! { <PillBottle /> }, "Pill Bottle"),
        (html! { <Pin /> }, "Pin"),
        (html! { <PinOff /> }, "Pin Off"),
        (html! { <Pipette /> }, "Pipette"),
        (html! { <Pizza /> }, "Pizza"),
        (html! { <Plane /> }, "Plane"),
        (html! { <PlaneLanding /> }, "Plane Landing"),
        (html! { <PlaneTakeoff /> }, "Plane Takeoff"),
        (html! { <Play /> }, "Play"),
        (html! { <Plug /> }, "Plug"),
        (html! { <Plug2 /> }, "Plug 2"),
        (html! { <PlugZap /> }, "Plug Zap"),
        (html! { <Plus /> }, "Plus"),
        (html! { <Pocket /> }, "Pocket"),
        (html! { <PocketKnife /> }, "Pocket Knife"),
        (html! { <Podcast /> }, "Podcast"),
        (html! { <Pointer /> }, "Pointer"),
        (html! { <PointerOff /> }, "Pointer Off"),
        (html! { <Popcorn /> }, "Popcorn"),
        (html! { <Popsicle /> }, "Popsicle"),
        (html! { <PoundSterling /> }, "Pound Sterling"),
        (html! { <Power /> }, "Power"),
        (html! { <PowerOff /> }, "Power Off"),
        (html! { <Presentation /> }, "Presentation"),
        (html! { <Printer /> }, "Printer"),
        (html! { <PrinterCheck /> }, "Printer Check"),
        (html! { <Projector /> }, "Projector"),
        (html! { <Proportions /> }, "Proportions"),
        (html! { <Puzzle /> }, "Puzzle"),
        (html! { <Pyramid /> }, "Pyramid"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsQ() -> Html {
    let icons = [
        (html! { <QrCode /> }, "Qr Code"),
        (html! { <Quote /> }, "Quote"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsR() -> Html {
    let icons = [
        (html! { <Rabbit /> }, "Rabbit"),
        (html! { <Radar /> }, "Radar"),
        (html! { <Radiation /> }, "Radiation"),
        (html! { <Radical /> }, "Radical"),
        (html! { <Radio /> }, "Radio"),
        (html! { <RadioReceiver /> }, "Radio Receiver"),
        (html! { <RadioTower /> }, "Radio Tower"),
        (html! { <Radius /> }, "Radius"),
        (html! { <RailSymbol /> }, "Rail Symbol"),
        (html! { <Rainbow /> }, "Rainbow"),
        (html! { <Rat /> }, "Rat"),
        (html! { <Ratio /> }, "Ratio"),
        (html! { <Receipt /> }, "Receipt"),
        (html! { <ReceiptCent /> }, "Receipt Cent"),
        (html! { <ReceiptEuro /> }, "Receipt Euro"),
        (html! { <ReceiptIndianRupee /> }, "Receipt Indian Rupee"),
        (html! { <ReceiptJapaneseYen /> }, "Receipt Japanese Yen"),
        (html! { <ReceiptPoundSterling /> }, "Receipt Pound Sterling"),
        (html! { <ReceiptRussianRuble /> }, "Receipt Russian Ruble"),
        (html! { <ReceiptSwissFranc /> }, "Receipt Swiss Franc"),
        (html! { <ReceiptText /> }, "Receipt Text"),
        (html! { <RectangleEllipsis /> }, "Rectangle Ellipsis"),
        (html! { <RectangleGoggles /> }, "Rectangle Goggles"),
        (html! { <RectangleHorizontal /> }, "Rectangle Horizontal"),
        (html! { <RectangleVertical /> }, "Rectangle Vertical"),
        (html! { <Recycle /> }, "Recycle"),
        (html! { <Redo /> }, "Redo"),
        (html! { <Redo2 /> }, "Redo 2"),
        (html! { <RedoDot /> }, "Redo Dot"),
        (html! { <RefreshCcw /> }, "Refresh Ccw"),
        (html! { <RefreshCcwDot /> }, "Refresh Ccw Dot"),
        (html! { <RefreshCw /> }, "Refresh Cw"),
        (html! { <RefreshCwOff /> }, "Refresh Cw Off"),
        (html! { <Refrigerator /> }, "Refrigerator"),
        (html! { <Regex /> }, "Regex"),
        (html! { <RemoveFormatting /> }, "Remove Formatting"),
        (html! { <Repeat /> }, "Repeat"),
        (html! { <Repeat1 /> }, "Repeat 1"),
        (html! { <Repeat2 /> }, "Repeat 2"),
        (html! { <Replace /> }, "Replace"),
        (html! { <ReplaceAll /> }, "Replace All"),
        (html! { <Reply /> }, "Reply"),
        (html! { <ReplyAll /> }, "Reply All"),
        (html! { <Rewind /> }, "Rewind"),
        (html! { <Ribbon /> }, "Ribbon"),
        (html! { <Rocket /> }, "Rocket"),
        (html! { <RockingChair /> }, "Rocking Chair"),
        (html! { <RollerCoaster /> }, "Roller Coaster"),
        (html! { <Rotate3D /> }, "Rotate 3 D"),
        (html! { <RotateCcw /> }, "Rotate Ccw"),
        (html! { <RotateCcwKey /> }, "Rotate Ccw Key"),
        (html! { <RotateCcwSquare /> }, "Rotate Ccw Square"),
        (html! { <RotateCw /> }, "Rotate Cw"),
        (html! { <RotateCwSquare /> }, "Rotate Cw Square"),
        (html! { <Route /> }, "Route"),
        (html! { <RouteOff /> }, "Route Off"),
        (html! { <Router /> }, "Router"),
        (html! { <Rows2 /> }, "Rows 2"),
        (html! { <Rows3 /> }, "Rows 3"),
        (html! { <Rows4 /> }, "Rows 4"),
        (html! { <Rss /> }, "Rss"),
        (html! { <Ruler /> }, "Ruler"),
        (html! { <RulerDimensionLine /> }, "Ruler Dimension Line"),
        (html! { <RussianRuble /> }, "Russian Ruble"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsS() -> Html {
    let icons = [
        (html! { <Sailboat /> }, "Sailboat"),
        (html! { <Salad /> }, "Salad"),
        (html! { <Sandwich /> }, "Sandwich"),
        (html! { <Satellite /> }, "Satellite"),
        (html! { <SatelliteDish /> }, "Satellite Dish"),
        (html! { <SaudiRiyal /> }, "Saudi Riyal"),
        (html! { <Save /> }, "Save"),
        (html! { <SaveAll /> }, "Save All"),
        (html! { <SaveOff /> }, "Save Off"),
        (html! { <Scale /> }, "Scale"),
        (html! { <Scale3D /> }, "Scale 3 D"),
        (html! { <Scaling /> }, "Scaling"),
        (html! { <Scan /> }, "Scan"),
        (html! { <ScanBarcode /> }, "Scan Barcode"),
        (html! { <ScanEye /> }, "Scan Eye"),
        (html! { <ScanFace /> }, "Scan Face"),
        (html! { <ScanHeart /> }, "Scan Heart"),
        (html! { <ScanLine /> }, "Scan Line"),
        (html! { <ScanQrCode /> }, "Scan Qr Code"),
        (html! { <ScanSearch /> }, "Scan Search"),
        (html! { <ScanText /> }, "Scan Text"),
        (html! { <School /> }, "School"),
        (html! { <Scissors /> }, "Scissors"),
        (html! { <ScissorsLineDashed /> }, "Scissors Line Dashed"),
        (html! { <ScreenShare /> }, "Screen Share"),
        (html! { <ScreenShareOff /> }, "Screen Share Off"),
        (html! { <Scroll /> }, "Scroll"),
        (html! { <ScrollText /> }, "Scroll Text"),
        (html! { <Search /> }, "Search"),
        (html! { <SearchCheck /> }, "Search Check"),
        (html! { <SearchCode /> }, "Search Code"),
        (html! { <SearchSlash /> }, "Search Slash"),
        (html! { <SearchX /> }, "Search X"),
        (html! { <Section /> }, "Section"),
        (html! { <Send /> }, "Send"),
        (html! { <SendHorizontal /> }, "Send Horizontal"),
        (html! { <SendToBack /> }, "Send To Back"),
        (html! { <SeparatorHorizontal /> }, "Separator Horizontal"),
        (html! { <SeparatorVertical /> }, "Separator Vertical"),
        (html! { <Server /> }, "Server"),
        (html! { <ServerCog /> }, "Server Cog"),
        (html! { <ServerCrash /> }, "Server Crash"),
        (html! { <ServerOff /> }, "Server Off"),
        (html! { <Settings /> }, "Settings"),
        (html! { <Settings2 /> }, "Settings 2"),
        (html! { <Shapes /> }, "Shapes"),
        (html! { <Share /> }, "Share"),
        (html! { <Share2 /> }, "Share 2"),
        (html! { <Sheet /> }, "Sheet"),
        (html! { <Shell /> }, "Shell"),
        (html! { <Shield /> }, "Shield"),
        (html! { <ShieldAlert /> }, "Shield Alert"),
        (html! { <ShieldBan /> }, "Shield Ban"),
        (html! { <ShieldCheck /> }, "Shield Check"),
        (html! { <ShieldEllipsis /> }, "Shield Ellipsis"),
        (html! { <ShieldHalf /> }, "Shield Half"),
        (html! { <ShieldMinus /> }, "Shield Minus"),
        (html! { <ShieldOff /> }, "Shield Off"),
        (html! { <ShieldPlus /> }, "Shield Plus"),
        (html! { <ShieldQuestion /> }, "Shield Question"),
        (html! { <ShieldUser /> }, "Shield User"),
        (html! { <ShieldX /> }, "Shield X"),
        (html! { <Ship /> }, "Ship"),
        (html! { <ShipWheel /> }, "Ship Wheel"),
        (html! { <Shirt /> }, "Shirt"),
        (html! { <ShoppingBag /> }, "Shopping Bag"),
        (html! { <ShoppingBasket /> }, "Shopping Basket"),
        (html! { <ShoppingCart /> }, "Shopping Cart"),
        (html! { <Shovel /> }, "Shovel"),
        (html! { <ShowerHead /> }, "Shower Head"),
        (html! { <Shredder /> }, "Shredder"),
        (html! { <Shrimp /> }, "Shrimp"),
        (html! { <Shrink /> }, "Shrink"),
        (html! { <Shrub /> }, "Shrub"),
        (html! { <Shuffle /> }, "Shuffle"),
        (html! { <Sigma /> }, "Sigma"),
        (html! { <Signal /> }, "Signal"),
        (html! { <SignalHigh /> }, "Signal High"),
        (html! { <SignalLow /> }, "Signal Low"),
        (html! { <SignalMedium /> }, "Signal Medium"),
        (html! { <SignalZero /> }, "Signal Zero"),
        (html! { <Signature /> }, "Signature"),
        (html! { <Signpost /> }, "Signpost"),
        (html! { <SignpostBig /> }, "Signpost Big"),
        (html! { <Siren /> }, "Siren"),
        (html! { <SkipBack /> }, "Skip Back"),
        (html! { <SkipForward /> }, "Skip Forward"),
        (html! { <Skull /> }, "Skull"),
        (html! { <Slack /> }, "Slack"),
        (html! { <Slash /> }, "Slash"),
        (html! { <Slice /> }, "Slice"),
        (html! { <SlidersHorizontal /> }, "Sliders Horizontal"),
        (html! { <SlidersVertical /> }, "Sliders Vertical"),
        (html! { <Smartphone /> }, "Smartphone"),
        (html! { <SmartphoneCharging /> }, "Smartphone Charging"),
        (html! { <SmartphoneNfc /> }, "Smartphone Nfc"),
        (html! { <Smile /> }, "Smile"),
        (html! { <SmilePlus /> }, "Smile Plus"),
        (html! { <Snail /> }, "Snail"),
        (html! { <Snowflake /> }, "Snowflake"),
        (html! { <SoapDispenserDroplet /> }, "Soap Dispenser Droplet"),
        (html! { <Sofa /> }, "Sofa"),
        (html! { <Soup /> }, "Soup"),
        (html! { <Space /> }, "Space"),
        (html! { <Spade /> }, "Spade"),
        (html! { <Sparkle /> }, "Sparkle"),
        (html! { <Sparkles /> }, "Sparkles"),
        (html! { <Speaker /> }, "Speaker"),
        (html! { <Speech /> }, "Speech"),
        (html! { <SpellCheck /> }, "Spell Check"),
        (html! { <SpellCheck2 /> }, "Spell Check 2"),
        (html! { <Spline /> }, "Spline"),
        (html! { <SplinePointer /> }, "Spline Pointer"),
        (html! { <Split /> }, "Split"),
        (html! { <SprayCan /> }, "Spray Can"),
        (html! { <Sprout /> }, "Sprout"),
        (html! { <Square /> }, "Square"),
        (html! { <SquareActivity /> }, "Square Activity"),
        (html! { <SquareArrowDown /> }, "Square Arrow Down"),
        (html! { <SquareArrowDownLeft /> }, "Square Arrow Down Left"),
        (
            html! { <SquareArrowDownRight /> },
            "Square Arrow Down Right",
        ),
        (html! { <SquareArrowLeft /> }, "Square Arrow Left"),
        (
            html! { <SquareArrowOutDownLeft /> },
            "Square Arrow Out Down Left",
        ),
        (
            html! { <SquareArrowOutDownRight /> },
            "Square Arrow Out Down Right",
        ),
        (
            html! { <SquareArrowOutUpLeft /> },
            "Square Arrow Out Up Left",
        ),
        (
            html! { <SquareArrowOutUpRight /> },
            "Square Arrow Out Up Right",
        ),
        (html! { <SquareArrowRight /> }, "Square Arrow Right"),
        (html! { <SquareArrowUp /> }, "Square Arrow Up"),
        (html! { <SquareArrowUpLeft /> }, "Square Arrow Up Left"),
        (html! { <SquareArrowUpRight /> }, "Square Arrow Up Right"),
        (html! { <SquareAsterisk /> }, "Square Asterisk"),
        (
            html! { <SquareBottomDashedScissors /> },
            "Square Bottom Dashed Scissors",
        ),
        (html! { <SquareChartGantt /> }, "Square Chart Gantt"),
        (html! { <SquareCheck /> }, "Square Check"),
        (html! { <SquareCheckBig /> }, "Square Check Big"),
        (html! { <SquareChevronDown /> }, "Square Chevron Down"),
        (html! { <SquareChevronLeft /> }, "Square Chevron Left"),
        (html! { <SquareChevronRight /> }, "Square Chevron Right"),
        (html! { <SquareChevronUp /> }, "Square Chevron Up"),
        (html! { <SquareCode /> }, "Square Code"),
        (html! { <SquareDashed /> }, "Square Dashed"),
        (html! { <SquareDashedBottom /> }, "Square Dashed Bottom"),
        (
            html! { <SquareDashedBottomCode /> },
            "Square Dashed Bottom Code",
        ),
        (html! { <SquareDashedKanban /> }, "Square Dashed Kanban"),
        (
            html! { <SquareDashedMousePointer /> },
            "Square Dashed Mouse Pointer",
        ),
        (
            html! { <SquareDashedTopSolid /> },
            "Square Dashed Top Solid",
        ),
        (html! { <SquareDivide /> }, "Square Divide"),
        (html! { <SquareDot /> }, "Square Dot"),
        (html! { <SquareEqual /> }, "Square Equal"),
        (html! { <SquareFunction /> }, "Square Function"),
        (html! { <SquareKanban /> }, "Square Kanban"),
        (html! { <SquareLibrary /> }, "Square Library"),
        (html! { <SquareM /> }, "Square M"),
        (html! { <SquareMenu /> }, "Square Menu"),
        (html! { <SquareMinus /> }, "Square Minus"),
        (html! { <SquareMousePointer /> }, "Square Mouse Pointer"),
        (html! { <SquareParking /> }, "Square Parking"),
        (html! { <SquareParkingOff /> }, "Square Parking Off"),
        (html! { <SquarePen /> }, "Square Pen"),
        (html! { <SquarePercent /> }, "Square Percent"),
        (html! { <SquarePi /> }, "Square Pi"),
        (html! { <SquarePilcrow /> }, "Square Pilcrow"),
        (html! { <SquarePlay /> }, "Square Play"),
        (html! { <SquarePlus /> }, "Square Plus"),
        (html! { <SquarePower /> }, "Square Power"),
        (html! { <SquareRadical /> }, "Square Radical"),
        (html! { <SquareRoundCorner /> }, "Square Round Corner"),
        (html! { <SquareScissors /> }, "Square Scissors"),
        (html! { <SquareSigma /> }, "Square Sigma"),
        (html! { <SquareSlash /> }, "Square Slash"),
        (
            html! { <SquareSplitHorizontal /> },
            "Square Split Horizontal",
        ),
        (html! { <SquareSplitVertical /> }, "Square Split Vertical"),
        (html! { <SquareSquare /> }, "Square Square"),
        (html! { <SquareStack /> }, "Square Stack"),
        (html! { <SquareTerminal /> }, "Square Terminal"),
        (html! { <SquareUser /> }, "Square User"),
        (html! { <SquareUserRound /> }, "Square User Round"),
        (html! { <SquareX /> }, "Square X"),
        (html! { <SquaresExclude /> }, "Squares Exclude"),
        (html! { <SquaresIntersect /> }, "Squares Intersect"),
        (html! { <SquaresSubtract /> }, "Squares Subtract"),
        (html! { <SquaresUnite /> }, "Squares Unite"),
        (html! { <Squircle /> }, "Squircle"),
        (html! { <Squirrel /> }, "Squirrel"),
        (html! { <Stamp /> }, "Stamp"),
        (html! { <Star /> }, "Star"),
        (html! { <StarHalf /> }, "Star Half"),
        (html! { <StarOff /> }, "Star Off"),
        (html! { <StepBack /> }, "Step Back"),
        (html! { <StepForward /> }, "Step Forward"),
        (html! { <Stethoscope /> }, "Stethoscope"),
        (html! { <Sticker /> }, "Sticker"),
        (html! { <StickyNote /> }, "Sticky Note"),
        (html! { <Store /> }, "Store"),
        (html! { <StretchHorizontal /> }, "Stretch Horizontal"),
        (html! { <StretchVertical /> }, "Stretch Vertical"),
        (html! { <Strikethrough /> }, "Strikethrough"),
        (html! { <Subscript /> }, "Subscript"),
        (html! { <Sun /> }, "Sun"),
        (html! { <SunDim /> }, "Sun Dim"),
        (html! { <SunMedium /> }, "Sun Medium"),
        (html! { <SunMoon /> }, "Sun Moon"),
        (html! { <SunSnow /> }, "Sun Snow"),
        (html! { <Sunrise /> }, "Sunrise"),
        (html! { <Sunset /> }, "Sunset"),
        (html! { <Superscript /> }, "Superscript"),
        (html! { <SwatchBook /> }, "Swatch Book"),
        (html! { <SwissFranc /> }, "Swiss Franc"),
        (html! { <SwitchCamera /> }, "Switch Camera"),
        (html! { <Sword /> }, "Sword"),
        (html! { <Swords /> }, "Swords"),
        (html! { <Syringe /> }, "Syringe"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsT() -> Html {
    let icons = [
        (html! { <Table /> }, "Table"),
        (html! { <Table2 /> }, "Table 2"),
        (html! { <TableCellsMerge /> }, "Table Cells Merge"),
        (html! { <TableCellsSplit /> }, "Table Cells Split"),
        (html! { <TableColumnsSplit /> }, "Table Columns Split"),
        (html! { <TableOfContents /> }, "Table Of Contents"),
        (html! { <TableProperties /> }, "Table Properties"),
        (html! { <TableRowsSplit /> }, "Table Rows Split"),
        (html! { <Tablet /> }, "Tablet"),
        (html! { <TabletSmartphone /> }, "Tablet Smartphone"),
        (html! { <Tablets /> }, "Tablets"),
        (html! { <Tag /> }, "Tag"),
        (html! { <Tags /> }, "Tags"),
        (html! { <Tally1 /> }, "Tally 1"),
        (html! { <Tally2 /> }, "Tally 2"),
        (html! { <Tally3 /> }, "Tally 3"),
        (html! { <Tally4 /> }, "Tally 4"),
        (html! { <Tally5 /> }, "Tally 5"),
        (html! { <Tangent /> }, "Tangent"),
        (html! { <Target /> }, "Target"),
        (html! { <Telescope /> }, "Telescope"),
        (html! { <Tent /> }, "Tent"),
        (html! { <TentTree /> }, "Tent Tree"),
        (html! { <Terminal /> }, "Terminal"),
        (html! { <TestTube /> }, "Test Tube"),
        (html! { <TestTubeDiagonal /> }, "Test Tube Diagonal"),
        (html! { <TestTubes /> }, "Test Tubes"),
        (html! { <Text /> }, "Text"),
        (html! { <TextCursor /> }, "Text Cursor"),
        (html! { <TextCursorInput /> }, "Text Cursor Input"),
        (html! { <TextQuote /> }, "Text Quote"),
        (html! { <TextSearch /> }, "Text Search"),
        (html! { <TextSelect /> }, "Text Select"),
        (html! { <Theater /> }, "Theater"),
        (html! { <Thermometer /> }, "Thermometer"),
        (html! { <ThermometerSnowflake /> }, "Thermometer Snowflake"),
        (html! { <ThermometerSun /> }, "Thermometer Sun"),
        (html! { <ThumbsDown /> }, "Thumbs Down"),
        (html! { <ThumbsUp /> }, "Thumbs Up"),
        (html! { <Ticket /> }, "Ticket"),
        (html! { <TicketCheck /> }, "Ticket Check"),
        (html! { <TicketMinus /> }, "Ticket Minus"),
        (html! { <TicketPercent /> }, "Ticket Percent"),
        (html! { <TicketPlus /> }, "Ticket Plus"),
        (html! { <TicketSlash /> }, "Ticket Slash"),
        (html! { <TicketX /> }, "Ticket X"),
        (html! { <Tickets /> }, "Tickets"),
        (html! { <TicketsPlane /> }, "Tickets Plane"),
        (html! { <Timer /> }, "Timer"),
        (html! { <TimerOff /> }, "Timer Off"),
        (html! { <TimerReset /> }, "Timer Reset"),
        (html! { <ToggleLeft /> }, "Toggle Left"),
        (html! { <ToggleRight /> }, "Toggle Right"),
        (html! { <Toilet /> }, "Toilet"),
        (html! { <Tornado /> }, "Tornado"),
        (html! { <Torus /> }, "Torus"),
        (html! { <Touchpad /> }, "Touchpad"),
        (html! { <TouchpadOff /> }, "Touchpad Off"),
        (html! { <TowerControl /> }, "Tower Control"),
        (html! { <ToyBrick /> }, "Toy Brick"),
        (html! { <Tractor /> }, "Tractor"),
        (html! { <TrafficCone /> }, "Traffic Cone"),
        (html! { <TrainFront /> }, "Train Front"),
        (html! { <TrainFrontTunnel /> }, "Train Front Tunnel"),
        (html! { <TrainTrack /> }, "Train Track"),
        (html! { <TramFront /> }, "Tram Front"),
        (html! { <Transgender /> }, "Transgender"),
        (html! { <Trash /> }, "Trash"),
        (html! { <Trash2 /> }, "Trash 2"),
        (html! { <TreeDeciduous /> }, "Tree Deciduous"),
        (html! { <TreePalm /> }, "Tree Palm"),
        (html! { <TreePine /> }, "Tree Pine"),
        (html! { <Trees /> }, "Trees"),
        (html! { <Trello /> }, "Trello"),
        (html! { <TrendingDown /> }, "Trending Down"),
        (html! { <TrendingUp /> }, "Trending Up"),
        (html! { <TrendingUpDown /> }, "Trending Up Down"),
        (html! { <Triangle /> }, "Triangle"),
        (html! { <TriangleAlert /> }, "Triangle Alert"),
        (html! { <TriangleDashed /> }, "Triangle Dashed"),
        (html! { <TriangleRight /> }, "Triangle Right"),
        (html! { <Trophy /> }, "Trophy"),
        (html! { <Truck /> }, "Truck"),
        (html! { <TruckElectric /> }, "Truck Electric"),
        (html! { <Turtle /> }, "Turtle"),
        (html! { <Tv /> }, "Tv"),
        (html! { <TvMinimal /> }, "Tv Minimal"),
        (html! { <TvMinimalPlay /> }, "Tv Minimal Play"),
        (html! { <Twitch /> }, "Twitch"),
        (html! { <Twitter /> }, "Twitter"),
        (html! { <Type /> }, "Type"),
        (html! { <TypeOutline /> }, "Type Outline"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsU() -> Html {
    let icons = [
        (html! { <Umbrella /> }, "Umbrella"),
        (html! { <UmbrellaOff /> }, "Umbrella Off"),
        (html! { <Underline /> }, "Underline"),
        (html! { <Undo /> }, "Undo"),
        (html! { <Undo2 /> }, "Undo 2"),
        (html! { <UndoDot /> }, "Undo Dot"),
        (html! { <UnfoldHorizontal /> }, "Unfold Horizontal"),
        (html! { <UnfoldVertical /> }, "Unfold Vertical"),
        (html! { <Ungroup /> }, "Ungroup"),
        (html! { <University /> }, "University"),
        (html! { <Unlink /> }, "Unlink"),
        (html! { <Unlink2 /> }, "Unlink 2"),
        (html! { <Unplug /> }, "Unplug"),
        (html! { <Upload /> }, "Upload"),
        (html! { <Usb /> }, "Usb"),
        (html! { <User /> }, "User"),
        (html! { <UserCheck /> }, "User Check"),
        (html! { <UserCog /> }, "User Cog"),
        (html! { <UserLock /> }, "User Lock"),
        (html! { <UserMinus /> }, "User Minus"),
        (html! { <UserPen /> }, "User Pen"),
        (html! { <UserPlus /> }, "User Plus"),
        (html! { <UserRound /> }, "User Round"),
        (html! { <UserRoundCheck /> }, "User Round Check"),
        (html! { <UserRoundCog /> }, "User Round Cog"),
        (html! { <UserRoundMinus /> }, "User Round Minus"),
        (html! { <UserRoundPen /> }, "User Round Pen"),
        (html! { <UserRoundPlus /> }, "User Round Plus"),
        (html! { <UserRoundSearch /> }, "User Round Search"),
        (html! { <UserRoundX /> }, "User Round X"),
        (html! { <UserSearch /> }, "User Search"),
        (html! { <UserX /> }, "User X"),
        (html! { <Users /> }, "Users"),
        (html! { <UsersRound /> }, "Users Round"),
        (html! { <Utensils /> }, "Utensils"),
        (html! { <UtensilsCrossed /> }, "Utensils Crossed"),
        (html! { <UtilityPole /> }, "Utility Pole"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsV() -> Html {
    let icons = [
        (html! { <Variable /> }, "Variable"),
        (html! { <Vault /> }, "Vault"),
        (html! { <Vegan /> }, "Vegan"),
        (html! { <VenetianMask /> }, "Venetian Mask"),
        (html! { <Venus /> }, "Venus"),
        (html! { <VenusAndMars /> }, "Venus And Mars"),
        (html! { <Vibrate /> }, "Vibrate"),
        (html! { <VibrateOff /> }, "Vibrate Off"),
        (html! { <Video /> }, "Video"),
        (html! { <VideoOff /> }, "Video Off"),
        (html! { <Videotape /> }, "Videotape"),
        (html! { <View /> }, "View"),
        (html! { <Voicemail /> }, "Voicemail"),
        (html! { <Volleyball /> }, "Volleyball"),
        (html! { <Volume /> }, "Volume"),
        (html! { <Volume1 /> }, "Volume 1"),
        (html! { <Volume2 /> }, "Volume 2"),
        (html! { <VolumeOff /> }, "Volume Off"),
        (html! { <VolumeX /> }, "Volume X"),
        (html! { <Vote /> }, "Vote"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsW() -> Html {
    let icons = [
        (html! { <Wallet /> }, "Wallet"),
        (html! { <WalletCards /> }, "Wallet Cards"),
        (html! { <WalletMinimal /> }, "Wallet Minimal"),
        (html! { <Wallpaper /> }, "Wallpaper"),
        (html! { <Wand /> }, "Wand"),
        (html! { <WandSparkles /> }, "Wand Sparkles"),
        (html! { <Warehouse /> }, "Warehouse"),
        (html! { <WashingMachine /> }, "Washing Machine"),
        (html! { <Watch /> }, "Watch"),
        (html! { <Waves /> }, "Waves"),
        (html! { <WavesLadder /> }, "Waves Ladder"),
        (html! { <Waypoints /> }, "Waypoints"),
        (html! { <Webcam /> }, "Webcam"),
        (html! { <Webhook /> }, "Webhook"),
        (html! { <WebhookOff /> }, "Webhook Off"),
        (html! { <Weight /> }, "Weight"),
        (html! { <Wheat /> }, "Wheat"),
        (html! { <WheatOff /> }, "Wheat Off"),
        (html! { <WholeWord /> }, "Whole Word"),
        (html! { <Wifi /> }, "Wifi"),
        (html! { <WifiHigh /> }, "Wifi High"),
        (html! { <WifiLow /> }, "Wifi Low"),
        (html! { <WifiOff /> }, "Wifi Off"),
        (html! { <WifiPen /> }, "Wifi Pen"),
        (html! { <WifiZero /> }, "Wifi Zero"),
        (html! { <Wind /> }, "Wind"),
        (html! { <WindArrowDown /> }, "Wind Arrow Down"),
        (html! { <Wine /> }, "Wine"),
        (html! { <WineOff /> }, "Wine Off"),
        (html! { <Workflow /> }, "Workflow"),
        (html! { <Worm /> }, "Worm"),
        (html! { <WrapText /> }, "Wrap Text"),
        (html! { <Wrench /> }, "Wrench"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsX() -> Html {
    let icons = [(html! { <X /> }, "X")];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsY() -> Html {
    let icons = [(html! { <Youtube /> }, "Youtube")];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsZ() -> Html {
    let icons = [
        (html! { <Zap /> }, "Zap"),
        (html! { <ZapOff /> }, "Zap Off"),
        (html! { <ZoomIn /> }, "Zoom In"),
        (html! { <ZoomOut /> }, "Zoom Out"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
