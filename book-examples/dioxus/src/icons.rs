use dioxus::prelude::*;
use lucide_dioxus::*;
#[component]
pub fn Icons() -> Element {
    rsx! {
        div { class: "w-full max-w-80 py-4",
            IconsA1 {}
            IconsA2 {}
            IconsB1 {}
            IconsB2 {}
            IconsC1 {}
            IconsC2 {}
            IconsC3 {}
            IconsD1 {}
            IconsE1 {}
            IconsF1 {}
            IconsF2 {}
            IconsG1 {}
            IconsH1 {}
            IconsI1 {}
            IconsJ1 {}
            IconsK1 {}
            IconsL1 {}
            IconsM1 {}
            IconsM2 {}
            IconsN1 {}
            IconsO1 {}
            IconsP1 {}
            IconsQ1 {}
            IconsR1 {}
            IconsS1 {}
            IconsS2 {}
            IconsS3 {}
            IconsT1 {}
            IconsU1 {}
            IconsV1 {}
            IconsW1 {}
            IconsX1 {}
            IconsY1 {}
            IconsZ1 {}
        }
    }
}
#[component]
pub fn IconsA1() -> Element {
    let icons = [
        (
            rsx! {
                AArrowDown {}
            },
            "A Arrow Down",
        ),
        (
            rsx! {
                AArrowUp {}
            },
            "A Arrow Up",
        ),
        (
            rsx! {
                ALargeSmall {}
            },
            "A Large Small",
        ),
        (
            rsx! {
                Accessibility {}
            },
            "Accessibility",
        ),
        (
            rsx! {
                Activity {}
            },
            "Activity",
        ),
        (
            rsx! {
                AirVent {}
            },
            "Air Vent",
        ),
        (
            rsx! {
                Airplay {}
            },
            "Airplay",
        ),
        (
            rsx! {
                AlarmClock {}
            },
            "Alarm Clock",
        ),
        (
            rsx! {
                AlarmClockCheck {}
            },
            "Alarm Clock Check",
        ),
        (
            rsx! {
                AlarmClockMinus {}
            },
            "Alarm Clock Minus",
        ),
        (
            rsx! {
                AlarmClockOff {}
            },
            "Alarm Clock Off",
        ),
        (
            rsx! {
                AlarmClockPlus {}
            },
            "Alarm Clock Plus",
        ),
        (
            rsx! {
                AlarmSmoke {}
            },
            "Alarm Smoke",
        ),
        (
            rsx! {
                Album {}
            },
            "Album",
        ),
        (
            rsx! {
                AlignCenter {}
            },
            "Align Center",
        ),
        (
            rsx! {
                AlignCenterHorizontal {}
            },
            "Align Center Horizontal",
        ),
        (
            rsx! {
                AlignCenterVertical {}
            },
            "Align Center Vertical",
        ),
        (
            rsx! {
                AlignEndHorizontal {}
            },
            "Align End Horizontal",
        ),
        (
            rsx! {
                AlignEndVertical {}
            },
            "Align End Vertical",
        ),
        (
            rsx! {
                AlignHorizontalDistributeCenter {}
            },
            "Align Horizontal Distribute Center",
        ),
        (
            rsx! {
                AlignHorizontalDistributeEnd {}
            },
            "Align Horizontal Distribute End",
        ),
        (
            rsx! {
                AlignHorizontalDistributeStart {}
            },
            "Align Horizontal Distribute Start",
        ),
        (
            rsx! {
                AlignHorizontalJustifyCenter {}
            },
            "Align Horizontal Justify Center",
        ),
        (
            rsx! {
                AlignHorizontalJustifyEnd {}
            },
            "Align Horizontal Justify End",
        ),
        (
            rsx! {
                AlignHorizontalJustifyStart {}
            },
            "Align Horizontal Justify Start",
        ),
        (
            rsx! {
                AlignHorizontalSpaceAround {}
            },
            "Align Horizontal Space Around",
        ),
        (
            rsx! {
                AlignHorizontalSpaceBetween {}
            },
            "Align Horizontal Space Between",
        ),
        (
            rsx! {
                AlignJustify {}
            },
            "Align Justify",
        ),
        (
            rsx! {
                AlignLeft {}
            },
            "Align Left",
        ),
        (
            rsx! {
                AlignRight {}
            },
            "Align Right",
        ),
        (
            rsx! {
                AlignStartHorizontal {}
            },
            "Align Start Horizontal",
        ),
        (
            rsx! {
                AlignStartVertical {}
            },
            "Align Start Vertical",
        ),
        (
            rsx! {
                AlignVerticalDistributeCenter {}
            },
            "Align Vertical Distribute Center",
        ),
        (
            rsx! {
                AlignVerticalDistributeEnd {}
            },
            "Align Vertical Distribute End",
        ),
        (
            rsx! {
                AlignVerticalDistributeStart {}
            },
            "Align Vertical Distribute Start",
        ),
        (
            rsx! {
                AlignVerticalJustifyCenter {}
            },
            "Align Vertical Justify Center",
        ),
        (
            rsx! {
                AlignVerticalJustifyEnd {}
            },
            "Align Vertical Justify End",
        ),
        (
            rsx! {
                AlignVerticalJustifyStart {}
            },
            "Align Vertical Justify Start",
        ),
        (
            rsx! {
                AlignVerticalSpaceAround {}
            },
            "Align Vertical Space Around",
        ),
        (
            rsx! {
                AlignVerticalSpaceBetween {}
            },
            "Align Vertical Space Between",
        ),
        (
            rsx! {
                Ambulance {}
            },
            "Ambulance",
        ),
        (
            rsx! {
                Ampersand {}
            },
            "Ampersand",
        ),
        (
            rsx! {
                Ampersands {}
            },
            "Ampersands",
        ),
        (
            rsx! {
                Amphora {}
            },
            "Amphora",
        ),
        (
            rsx! {
                Anchor {}
            },
            "Anchor",
        ),
        (
            rsx! {
                Angry {}
            },
            "Angry",
        ),
        (
            rsx! {
                Annoyed {}
            },
            "Annoyed",
        ),
        (
            rsx! {
                Antenna {}
            },
            "Antenna",
        ),
        (
            rsx! {
                Anvil {}
            },
            "Anvil",
        ),
        (
            rsx! {
                Aperture {}
            },
            "Aperture",
        ),
        (
            rsx! {
                AppWindow {}
            },
            "App Window",
        ),
        (
            rsx! {
                AppWindowMac {}
            },
            "App Window Mac",
        ),
        (
            rsx! {
                Apple {}
            },
            "Apple",
        ),
        (
            rsx! {
                Archive {}
            },
            "Archive",
        ),
        (
            rsx! {
                ArchiveRestore {}
            },
            "Archive Restore",
        ),
        (
            rsx! {
                ArchiveX {}
            },
            "Archive X",
        ),
        (
            rsx! {
                Armchair {}
            },
            "Armchair",
        ),
        (
            rsx! {
                ArrowBigDown {}
            },
            "Arrow Big Down",
        ),
        (
            rsx! {
                ArrowBigDownDash {}
            },
            "Arrow Big Down Dash",
        ),
        (
            rsx! {
                ArrowBigLeft {}
            },
            "Arrow Big Left",
        ),
        (
            rsx! {
                ArrowBigLeftDash {}
            },
            "Arrow Big Left Dash",
        ),
        (
            rsx! {
                ArrowBigRight {}
            },
            "Arrow Big Right",
        ),
        (
            rsx! {
                ArrowBigRightDash {}
            },
            "Arrow Big Right Dash",
        ),
        (
            rsx! {
                ArrowBigUp {}
            },
            "Arrow Big Up",
        ),
        (
            rsx! {
                ArrowBigUpDash {}
            },
            "Arrow Big Up Dash",
        ),
        (
            rsx! {
                ArrowDown {}
            },
            "Arrow Down",
        ),
        (
            rsx! {
                ArrowDown01 {}
            },
            "Arrow Down 01",
        ),
        (
            rsx! {
                ArrowDown10 {}
            },
            "Arrow Down 10",
        ),
        (
            rsx! {
                ArrowDownAZ {}
            },
            "Arrow Down Az",
        ),
        (
            rsx! {
                ArrowDownFromLine {}
            },
            "Arrow Down From Line",
        ),
        (
            rsx! {
                ArrowDownLeft {}
            },
            "Arrow Down Left",
        ),
        (
            rsx! {
                ArrowDownNarrowWide {}
            },
            "Arrow Down Narrow Wide",
        ),
        (
            rsx! {
                ArrowDownRight {}
            },
            "Arrow Down Right",
        ),
        (
            rsx! {
                ArrowDownToDot {}
            },
            "Arrow Down To Dot",
        ),
        (
            rsx! {
                ArrowDownToLine {}
            },
            "Arrow Down To Line",
        ),
        (
            rsx! {
                ArrowDownUp {}
            },
            "Arrow Down Up",
        ),
        (
            rsx! {
                ArrowDownWideNarrow {}
            },
            "Arrow Down Wide Narrow",
        ),
        (
            rsx! {
                ArrowDownZA {}
            },
            "Arrow Down Za",
        ),
        (
            rsx! {
                ArrowLeft {}
            },
            "Arrow Left",
        ),
        (
            rsx! {
                ArrowLeftFromLine {}
            },
            "Arrow Left From Line",
        ),
        (
            rsx! {
                ArrowLeftRight {}
            },
            "Arrow Left Right",
        ),
        (
            rsx! {
                ArrowLeftToLine {}
            },
            "Arrow Left To Line",
        ),
        (
            rsx! {
                ArrowRight {}
            },
            "Arrow Right",
        ),
        (
            rsx! {
                ArrowRightFromLine {}
            },
            "Arrow Right From Line",
        ),
        (
            rsx! {
                ArrowRightLeft {}
            },
            "Arrow Right Left",
        ),
        (
            rsx! {
                ArrowRightToLine {}
            },
            "Arrow Right To Line",
        ),
        (
            rsx! {
                ArrowUp {}
            },
            "Arrow Up",
        ),
        (
            rsx! {
                ArrowUp01 {}
            },
            "Arrow Up 01",
        ),
        (
            rsx! {
                ArrowUp10 {}
            },
            "Arrow Up 10",
        ),
        (
            rsx! {
                ArrowUpAZ {}
            },
            "Arrow Up Az",
        ),
        (
            rsx! {
                ArrowUpDown {}
            },
            "Arrow Up Down",
        ),
        (
            rsx! {
                ArrowUpFromDot {}
            },
            "Arrow Up From Dot",
        ),
        (
            rsx! {
                ArrowUpFromLine {}
            },
            "Arrow Up From Line",
        ),
        (
            rsx! {
                ArrowUpLeft {}
            },
            "Arrow Up Left",
        ),
        (
            rsx! {
                ArrowUpNarrowWide {}
            },
            "Arrow Up Narrow Wide",
        ),
        (
            rsx! {
                ArrowUpRight {}
            },
            "Arrow Up Right",
        ),
        (
            rsx! {
                ArrowUpToLine {}
            },
            "Arrow Up To Line",
        ),
        (
            rsx! {
                ArrowUpWideNarrow {}
            },
            "Arrow Up Wide Narrow",
        ),
        (
            rsx! {
                ArrowUpZA {}
            },
            "Arrow Up Za",
        ),
        (
            rsx! {
                ArrowsUpFromLine {}
            },
            "Arrows Up From Line",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsA2() -> Element {
    let icons = [
        (
            rsx! {
                Asterisk {}
            },
            "Asterisk",
        ),
        (
            rsx! {
                AtSign {}
            },
            "At Sign",
        ),
        (
            rsx! {
                Atom {}
            },
            "Atom",
        ),
        (
            rsx! {
                AudioLines {}
            },
            "Audio Lines",
        ),
        (
            rsx! {
                AudioWaveform {}
            },
            "Audio Waveform",
        ),
        (
            rsx! {
                Award {}
            },
            "Award",
        ),
        (
            rsx! {
                Axe {}
            },
            "Axe",
        ),
        (
            rsx! {
                Axis3D {}
            },
            "Axis 3 D",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsB1() -> Element {
    let icons = [
        (
            rsx! {
                Baby {}
            },
            "Baby",
        ),
        (
            rsx! {
                Backpack {}
            },
            "Backpack",
        ),
        (
            rsx! {
                Badge {}
            },
            "Badge",
        ),
        (
            rsx! {
                BadgeAlert {}
            },
            "Badge Alert",
        ),
        (
            rsx! {
                BadgeCent {}
            },
            "Badge Cent",
        ),
        (
            rsx! {
                BadgeCheck {}
            },
            "Badge Check",
        ),
        (
            rsx! {
                BadgeDollarSign {}
            },
            "Badge Dollar Sign",
        ),
        (
            rsx! {
                BadgeEuro {}
            },
            "Badge Euro",
        ),
        (
            rsx! {
                BadgeHelp {}
            },
            "Badge Help",
        ),
        (
            rsx! {
                BadgeIndianRupee {}
            },
            "Badge Indian Rupee",
        ),
        (
            rsx! {
                BadgeInfo {}
            },
            "Badge Info",
        ),
        (
            rsx! {
                BadgeJapaneseYen {}
            },
            "Badge Japanese Yen",
        ),
        (
            rsx! {
                BadgeMinus {}
            },
            "Badge Minus",
        ),
        (
            rsx! {
                BadgePercent {}
            },
            "Badge Percent",
        ),
        (
            rsx! {
                BadgePlus {}
            },
            "Badge Plus",
        ),
        (
            rsx! {
                BadgePoundSterling {}
            },
            "Badge Pound Sterling",
        ),
        (
            rsx! {
                BadgeRussianRuble {}
            },
            "Badge Russian Ruble",
        ),
        (
            rsx! {
                BadgeSwissFranc {}
            },
            "Badge Swiss Franc",
        ),
        (
            rsx! {
                BadgeX {}
            },
            "Badge X",
        ),
        (
            rsx! {
                BaggageClaim {}
            },
            "Baggage Claim",
        ),
        (
            rsx! {
                Ban {}
            },
            "Ban",
        ),
        (
            rsx! {
                Banana {}
            },
            "Banana",
        ),
        (
            rsx! {
                Bandage {}
            },
            "Bandage",
        ),
        (
            rsx! {
                Banknote {}
            },
            "Banknote",
        ),
        (
            rsx! {
                BanknoteArrowDown {}
            },
            "Banknote Arrow Down",
        ),
        (
            rsx! {
                BanknoteArrowUp {}
            },
            "Banknote Arrow Up",
        ),
        (
            rsx! {
                BanknoteX {}
            },
            "Banknote X",
        ),
        (
            rsx! {
                Barcode {}
            },
            "Barcode",
        ),
        (
            rsx! {
                Baseline {}
            },
            "Baseline",
        ),
        (
            rsx! {
                Bath {}
            },
            "Bath",
        ),
        (
            rsx! {
                Battery {}
            },
            "Battery",
        ),
        (
            rsx! {
                BatteryCharging {}
            },
            "Battery Charging",
        ),
        (
            rsx! {
                BatteryFull {}
            },
            "Battery Full",
        ),
        (
            rsx! {
                BatteryLow {}
            },
            "Battery Low",
        ),
        (
            rsx! {
                BatteryMedium {}
            },
            "Battery Medium",
        ),
        (
            rsx! {
                BatteryPlus {}
            },
            "Battery Plus",
        ),
        (
            rsx! {
                BatteryWarning {}
            },
            "Battery Warning",
        ),
        (
            rsx! {
                Beaker {}
            },
            "Beaker",
        ),
        (
            rsx! {
                Bean {}
            },
            "Bean",
        ),
        (
            rsx! {
                BeanOff {}
            },
            "Bean Off",
        ),
        (
            rsx! {
                Bed {}
            },
            "Bed",
        ),
        (
            rsx! {
                BedDouble {}
            },
            "Bed Double",
        ),
        (
            rsx! {
                BedSingle {}
            },
            "Bed Single",
        ),
        (
            rsx! {
                Beef {}
            },
            "Beef",
        ),
        (
            rsx! {
                Beer {}
            },
            "Beer",
        ),
        (
            rsx! {
                BeerOff {}
            },
            "Beer Off",
        ),
        (
            rsx! {
                Bell {}
            },
            "Bell",
        ),
        (
            rsx! {
                BellDot {}
            },
            "Bell Dot",
        ),
        (
            rsx! {
                BellElectric {}
            },
            "Bell Electric",
        ),
        (
            rsx! {
                BellMinus {}
            },
            "Bell Minus",
        ),
        (
            rsx! {
                BellOff {}
            },
            "Bell Off",
        ),
        (
            rsx! {
                BellPlus {}
            },
            "Bell Plus",
        ),
        (
            rsx! {
                BellRing {}
            },
            "Bell Ring",
        ),
        (
            rsx! {
                BetweenHorizontalEnd {}
            },
            "Between Horizontal End",
        ),
        (
            rsx! {
                BetweenHorizontalStart {}
            },
            "Between Horizontal Start",
        ),
        (
            rsx! {
                BetweenVerticalEnd {}
            },
            "Between Vertical End",
        ),
        (
            rsx! {
                BetweenVerticalStart {}
            },
            "Between Vertical Start",
        ),
        (
            rsx! {
                BicepsFlexed {}
            },
            "Biceps Flexed",
        ),
        (
            rsx! {
                Bike {}
            },
            "Bike",
        ),
        (
            rsx! {
                Binary {}
            },
            "Binary",
        ),
        (
            rsx! {
                Binoculars {}
            },
            "Binoculars",
        ),
        (
            rsx! {
                Biohazard {}
            },
            "Biohazard",
        ),
        (
            rsx! {
                Bird {}
            },
            "Bird",
        ),
        (
            rsx! {
                Bitcoin {}
            },
            "Bitcoin",
        ),
        (
            rsx! {
                Blend {}
            },
            "Blend",
        ),
        (
            rsx! {
                Blinds {}
            },
            "Blinds",
        ),
        (
            rsx! {
                Blocks {}
            },
            "Blocks",
        ),
        (
            rsx! {
                Bluetooth {}
            },
            "Bluetooth",
        ),
        (
            rsx! {
                BluetoothConnected {}
            },
            "Bluetooth Connected",
        ),
        (
            rsx! {
                BluetoothOff {}
            },
            "Bluetooth Off",
        ),
        (
            rsx! {
                BluetoothSearching {}
            },
            "Bluetooth Searching",
        ),
        (
            rsx! {
                Bold {}
            },
            "Bold",
        ),
        (
            rsx! {
                Bolt {}
            },
            "Bolt",
        ),
        (
            rsx! {
                Bomb {}
            },
            "Bomb",
        ),
        (
            rsx! {
                Bone {}
            },
            "Bone",
        ),
        (
            rsx! {
                Book {}
            },
            "Book",
        ),
        (
            rsx! {
                BookA {}
            },
            "Book A",
        ),
        (
            rsx! {
                BookAudio {}
            },
            "Book Audio",
        ),
        (
            rsx! {
                BookCheck {}
            },
            "Book Check",
        ),
        (
            rsx! {
                BookCopy {}
            },
            "Book Copy",
        ),
        (
            rsx! {
                BookDashed {}
            },
            "Book Dashed",
        ),
        (
            rsx! {
                BookDown {}
            },
            "Book Down",
        ),
        (
            rsx! {
                BookHeadphones {}
            },
            "Book Headphones",
        ),
        (
            rsx! {
                BookHeart {}
            },
            "Book Heart",
        ),
        (
            rsx! {
                BookImage {}
            },
            "Book Image",
        ),
        (
            rsx! {
                BookKey {}
            },
            "Book Key",
        ),
        (
            rsx! {
                BookLock {}
            },
            "Book Lock",
        ),
        (
            rsx! {
                BookMarked {}
            },
            "Book Marked",
        ),
        (
            rsx! {
                BookMinus {}
            },
            "Book Minus",
        ),
        (
            rsx! {
                BookOpen {}
            },
            "Book Open",
        ),
        (
            rsx! {
                BookOpenCheck {}
            },
            "Book Open Check",
        ),
        (
            rsx! {
                BookOpenText {}
            },
            "Book Open Text",
        ),
        (
            rsx! {
                BookPlus {}
            },
            "Book Plus",
        ),
        (
            rsx! {
                BookText {}
            },
            "Book Text",
        ),
        (
            rsx! {
                BookType {}
            },
            "Book Type",
        ),
        (
            rsx! {
                BookUp {}
            },
            "Book Up",
        ),
        (
            rsx! {
                BookUp2 {}
            },
            "Book Up 2",
        ),
        (
            rsx! {
                BookUser {}
            },
            "Book User",
        ),
        (
            rsx! {
                BookX {}
            },
            "Book X",
        ),
        (
            rsx! {
                Bookmark {}
            },
            "Bookmark",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsB2() -> Element {
    let icons = [
        (
            rsx! {
                BookmarkCheck {}
            },
            "Bookmark Check",
        ),
        (
            rsx! {
                BookmarkMinus {}
            },
            "Bookmark Minus",
        ),
        (
            rsx! {
                BookmarkPlus {}
            },
            "Bookmark Plus",
        ),
        (
            rsx! {
                BookmarkX {}
            },
            "Bookmark X",
        ),
        (
            rsx! {
                BoomBox {}
            },
            "Boom Box",
        ),
        (
            rsx! {
                Bot {}
            },
            "Bot",
        ),
        (
            rsx! {
                BotMessageSquare {}
            },
            "Bot Message Square",
        ),
        (
            rsx! {
                BotOff {}
            },
            "Bot Off",
        ),
        (
            rsx! {
                BowArrow {}
            },
            "Bow Arrow",
        ),
        (
            rsx! {
                Box {}
            },
            "Box",
        ),
        (
            rsx! {
                Boxes {}
            },
            "Boxes",
        ),
        (
            rsx! {
                Braces {}
            },
            "Braces",
        ),
        (
            rsx! {
                Brackets {}
            },
            "Brackets",
        ),
        (
            rsx! {
                Brain {}
            },
            "Brain",
        ),
        (
            rsx! {
                BrainCircuit {}
            },
            "Brain Circuit",
        ),
        (
            rsx! {
                BrainCog {}
            },
            "Brain Cog",
        ),
        (
            rsx! {
                BrickWall {}
            },
            "Brick Wall",
        ),
        (
            rsx! {
                BrickWallFire {}
            },
            "Brick Wall Fire",
        ),
        (
            rsx! {
                Briefcase {}
            },
            "Briefcase",
        ),
        (
            rsx! {
                BriefcaseBusiness {}
            },
            "Briefcase Business",
        ),
        (
            rsx! {
                BriefcaseConveyorBelt {}
            },
            "Briefcase Conveyor Belt",
        ),
        (
            rsx! {
                BriefcaseMedical {}
            },
            "Briefcase Medical",
        ),
        (
            rsx! {
                BringToFront {}
            },
            "Bring To Front",
        ),
        (
            rsx! {
                Brush {}
            },
            "Brush",
        ),
        (
            rsx! {
                BrushCleaning {}
            },
            "Brush Cleaning",
        ),
        (
            rsx! {
                Bubbles {}
            },
            "Bubbles",
        ),
        (
            rsx! {
                Bug {}
            },
            "Bug",
        ),
        (
            rsx! {
                BugOff {}
            },
            "Bug Off",
        ),
        (
            rsx! {
                BugPlay {}
            },
            "Bug Play",
        ),
        (
            rsx! {
                Building {}
            },
            "Building",
        ),
        (
            rsx! {
                Building2 {}
            },
            "Building 2",
        ),
        (
            rsx! {
                Bus {}
            },
            "Bus",
        ),
        (
            rsx! {
                BusFront {}
            },
            "Bus Front",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsC1() -> Element {
    let icons = [
        (
            rsx! {
                Cable {}
            },
            "Cable",
        ),
        (
            rsx! {
                CableCar {}
            },
            "Cable Car",
        ),
        (
            rsx! {
                Cake {}
            },
            "Cake",
        ),
        (
            rsx! {
                CakeSlice {}
            },
            "Cake Slice",
        ),
        (
            rsx! {
                Calculator {}
            },
            "Calculator",
        ),
        (
            rsx! {
                Calendar {}
            },
            "Calendar",
        ),
        (
            rsx! {
                Calendar1 {}
            },
            "Calendar 1",
        ),
        (
            rsx! {
                CalendarArrowDown {}
            },
            "Calendar Arrow Down",
        ),
        (
            rsx! {
                CalendarArrowUp {}
            },
            "Calendar Arrow Up",
        ),
        (
            rsx! {
                CalendarCheck {}
            },
            "Calendar Check",
        ),
        (
            rsx! {
                CalendarCheck2 {}
            },
            "Calendar Check 2",
        ),
        (
            rsx! {
                CalendarClock {}
            },
            "Calendar Clock",
        ),
        (
            rsx! {
                CalendarCog {}
            },
            "Calendar Cog",
        ),
        (
            rsx! {
                CalendarDays {}
            },
            "Calendar Days",
        ),
        (
            rsx! {
                CalendarFold {}
            },
            "Calendar Fold",
        ),
        (
            rsx! {
                CalendarHeart {}
            },
            "Calendar Heart",
        ),
        (
            rsx! {
                CalendarMinus {}
            },
            "Calendar Minus",
        ),
        (
            rsx! {
                CalendarMinus2 {}
            },
            "Calendar Minus 2",
        ),
        (
            rsx! {
                CalendarOff {}
            },
            "Calendar Off",
        ),
        (
            rsx! {
                CalendarPlus {}
            },
            "Calendar Plus",
        ),
        (
            rsx! {
                CalendarPlus2 {}
            },
            "Calendar Plus 2",
        ),
        (
            rsx! {
                CalendarRange {}
            },
            "Calendar Range",
        ),
        (
            rsx! {
                CalendarSearch {}
            },
            "Calendar Search",
        ),
        (
            rsx! {
                CalendarSync {}
            },
            "Calendar Sync",
        ),
        (
            rsx! {
                CalendarX {}
            },
            "Calendar X",
        ),
        (
            rsx! {
                CalendarX2 {}
            },
            "Calendar X 2",
        ),
        (
            rsx! {
                Camera {}
            },
            "Camera",
        ),
        (
            rsx! {
                CameraOff {}
            },
            "Camera Off",
        ),
        (
            rsx! {
                Candy {}
            },
            "Candy",
        ),
        (
            rsx! {
                CandyCane {}
            },
            "Candy Cane",
        ),
        (
            rsx! {
                CandyOff {}
            },
            "Candy Off",
        ),
        (
            rsx! {
                Cannabis {}
            },
            "Cannabis",
        ),
        (
            rsx! {
                Captions {}
            },
            "Captions",
        ),
        (
            rsx! {
                CaptionsOff {}
            },
            "Captions Off",
        ),
        (
            rsx! {
                Car {}
            },
            "Car",
        ),
        (
            rsx! {
                CarFront {}
            },
            "Car Front",
        ),
        (
            rsx! {
                CarTaxiFront {}
            },
            "Car Taxi Front",
        ),
        (
            rsx! {
                Caravan {}
            },
            "Caravan",
        ),
        (
            rsx! {
                Carrot {}
            },
            "Carrot",
        ),
        (
            rsx! {
                CaseLower {}
            },
            "Case Lower",
        ),
        (
            rsx! {
                CaseSensitive {}
            },
            "Case Sensitive",
        ),
        (
            rsx! {
                CaseUpper {}
            },
            "Case Upper",
        ),
        (
            rsx! {
                CassetteTape {}
            },
            "Cassette Tape",
        ),
        (
            rsx! {
                Cast {}
            },
            "Cast",
        ),
        (
            rsx! {
                Castle {}
            },
            "Castle",
        ),
        (
            rsx! {
                Cat {}
            },
            "Cat",
        ),
        (
            rsx! {
                Cctv {}
            },
            "Cctv",
        ),
        (
            rsx! {
                ChartArea {}
            },
            "Chart Area",
        ),
        (
            rsx! {
                ChartBar {}
            },
            "Chart Bar",
        ),
        (
            rsx! {
                ChartBarBig {}
            },
            "Chart Bar Big",
        ),
        (
            rsx! {
                ChartBarDecreasing {}
            },
            "Chart Bar Decreasing",
        ),
        (
            rsx! {
                ChartBarIncreasing {}
            },
            "Chart Bar Increasing",
        ),
        (
            rsx! {
                ChartBarStacked {}
            },
            "Chart Bar Stacked",
        ),
        (
            rsx! {
                ChartCandlestick {}
            },
            "Chart Candlestick",
        ),
        (
            rsx! {
                ChartColumn {}
            },
            "Chart Column",
        ),
        (
            rsx! {
                ChartColumnBig {}
            },
            "Chart Column Big",
        ),
        (
            rsx! {
                ChartColumnDecreasing {}
            },
            "Chart Column Decreasing",
        ),
        (
            rsx! {
                ChartColumnIncreasing {}
            },
            "Chart Column Increasing",
        ),
        (
            rsx! {
                ChartColumnStacked {}
            },
            "Chart Column Stacked",
        ),
        (
            rsx! {
                ChartGantt {}
            },
            "Chart Gantt",
        ),
        (
            rsx! {
                ChartLine {}
            },
            "Chart Line",
        ),
        (
            rsx! {
                ChartNetwork {}
            },
            "Chart Network",
        ),
        (
            rsx! {
                ChartNoAxesColumn {}
            },
            "Chart No Axes Column",
        ),
        (
            rsx! {
                ChartNoAxesColumnDecreasing {}
            },
            "Chart No Axes Column Decreasing",
        ),
        (
            rsx! {
                ChartNoAxesColumnIncreasing {}
            },
            "Chart No Axes Column Increasing",
        ),
        (
            rsx! {
                ChartNoAxesCombined {}
            },
            "Chart No Axes Combined",
        ),
        (
            rsx! {
                ChartNoAxesGantt {}
            },
            "Chart No Axes Gantt",
        ),
        (
            rsx! {
                ChartPie {}
            },
            "Chart Pie",
        ),
        (
            rsx! {
                ChartScatter {}
            },
            "Chart Scatter",
        ),
        (
            rsx! {
                ChartSpline {}
            },
            "Chart Spline",
        ),
        (
            rsx! {
                Check {}
            },
            "Check",
        ),
        (
            rsx! {
                CheckCheck {}
            },
            "Check Check",
        ),
        (
            rsx! {
                ChefHat {}
            },
            "Chef Hat",
        ),
        (
            rsx! {
                Cherry {}
            },
            "Cherry",
        ),
        (
            rsx! {
                ChevronDown {}
            },
            "Chevron Down",
        ),
        (
            rsx! {
                ChevronFirst {}
            },
            "Chevron First",
        ),
        (
            rsx! {
                ChevronLast {}
            },
            "Chevron Last",
        ),
        (
            rsx! {
                ChevronLeft {}
            },
            "Chevron Left",
        ),
        (
            rsx! {
                ChevronRight {}
            },
            "Chevron Right",
        ),
        (
            rsx! {
                ChevronUp {}
            },
            "Chevron Up",
        ),
        (
            rsx! {
                ChevronsDown {}
            },
            "Chevrons Down",
        ),
        (
            rsx! {
                ChevronsDownUp {}
            },
            "Chevrons Down Up",
        ),
        (
            rsx! {
                ChevronsLeft {}
            },
            "Chevrons Left",
        ),
        (
            rsx! {
                ChevronsLeftRight {}
            },
            "Chevrons Left Right",
        ),
        (
            rsx! {
                ChevronsLeftRightEllipsis {}
            },
            "Chevrons Left Right Ellipsis",
        ),
        (
            rsx! {
                ChevronsRight {}
            },
            "Chevrons Right",
        ),
        (
            rsx! {
                ChevronsRightLeft {}
            },
            "Chevrons Right Left",
        ),
        (
            rsx! {
                ChevronsUp {}
            },
            "Chevrons Up",
        ),
        (
            rsx! {
                ChevronsUpDown {}
            },
            "Chevrons Up Down",
        ),
        (
            rsx! {
                Chrome {}
            },
            "Chrome",
        ),
        (
            rsx! {
                Church {}
            },
            "Church",
        ),
        (
            rsx! {
                Cigarette {}
            },
            "Cigarette",
        ),
        (
            rsx! {
                CigaretteOff {}
            },
            "Cigarette Off",
        ),
        (
            rsx! {
                Circle {}
            },
            "Circle",
        ),
        (
            rsx! {
                CircleAlert {}
            },
            "Circle Alert",
        ),
        (
            rsx! {
                CircleArrowDown {}
            },
            "Circle Arrow Down",
        ),
        (
            rsx! {
                CircleArrowLeft {}
            },
            "Circle Arrow Left",
        ),
        (
            rsx! {
                CircleArrowOutDownLeft {}
            },
            "Circle Arrow Out Down Left",
        ),
        (
            rsx! {
                CircleArrowOutDownRight {}
            },
            "Circle Arrow Out Down Right",
        ),
        (
            rsx! {
                CircleArrowOutUpLeft {}
            },
            "Circle Arrow Out Up Left",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsC2() -> Element {
    let icons = [
        (
            rsx! {
                CircleArrowOutUpRight {}
            },
            "Circle Arrow Out Up Right",
        ),
        (
            rsx! {
                CircleArrowRight {}
            },
            "Circle Arrow Right",
        ),
        (
            rsx! {
                CircleArrowUp {}
            },
            "Circle Arrow Up",
        ),
        (
            rsx! {
                CircleCheck {}
            },
            "Circle Check",
        ),
        (
            rsx! {
                CircleCheckBig {}
            },
            "Circle Check Big",
        ),
        (
            rsx! {
                CircleChevronDown {}
            },
            "Circle Chevron Down",
        ),
        (
            rsx! {
                CircleChevronLeft {}
            },
            "Circle Chevron Left",
        ),
        (
            rsx! {
                CircleChevronRight {}
            },
            "Circle Chevron Right",
        ),
        (
            rsx! {
                CircleChevronUp {}
            },
            "Circle Chevron Up",
        ),
        (
            rsx! {
                CircleDashed {}
            },
            "Circle Dashed",
        ),
        (
            rsx! {
                CircleDivide {}
            },
            "Circle Divide",
        ),
        (
            rsx! {
                CircleDollarSign {}
            },
            "Circle Dollar Sign",
        ),
        (
            rsx! {
                CircleDot {}
            },
            "Circle Dot",
        ),
        (
            rsx! {
                CircleDotDashed {}
            },
            "Circle Dot Dashed",
        ),
        (
            rsx! {
                CircleEllipsis {}
            },
            "Circle Ellipsis",
        ),
        (
            rsx! {
                CircleEqual {}
            },
            "Circle Equal",
        ),
        (
            rsx! {
                CircleFadingArrowUp {}
            },
            "Circle Fading Arrow Up",
        ),
        (
            rsx! {
                CircleFadingPlus {}
            },
            "Circle Fading Plus",
        ),
        (
            rsx! {
                CircleGauge {}
            },
            "Circle Gauge",
        ),
        (
            rsx! {
                CircleHelp {}
            },
            "Circle Help",
        ),
        (
            rsx! {
                CircleMinus {}
            },
            "Circle Minus",
        ),
        (
            rsx! {
                CircleOff {}
            },
            "Circle Off",
        ),
        (
            rsx! {
                CircleParking {}
            },
            "Circle Parking",
        ),
        (
            rsx! {
                CircleParkingOff {}
            },
            "Circle Parking Off",
        ),
        (
            rsx! {
                CirclePause {}
            },
            "Circle Pause",
        ),
        (
            rsx! {
                CirclePercent {}
            },
            "Circle Percent",
        ),
        (
            rsx! {
                CirclePlay {}
            },
            "Circle Play",
        ),
        (
            rsx! {
                CirclePlus {}
            },
            "Circle Plus",
        ),
        (
            rsx! {
                CirclePower {}
            },
            "Circle Power",
        ),
        (
            rsx! {
                CircleSlash {}
            },
            "Circle Slash",
        ),
        (
            rsx! {
                CircleSlash2 {}
            },
            "Circle Slash 2",
        ),
        (
            rsx! {
                CircleSmall {}
            },
            "Circle Small",
        ),
        (
            rsx! {
                CircleStop {}
            },
            "Circle Stop",
        ),
        (
            rsx! {
                CircleUser {}
            },
            "Circle User",
        ),
        (
            rsx! {
                CircleUserRound {}
            },
            "Circle User Round",
        ),
        (
            rsx! {
                CircleX {}
            },
            "Circle X",
        ),
        (
            rsx! {
                CircuitBoard {}
            },
            "Circuit Board",
        ),
        (
            rsx! {
                Citrus {}
            },
            "Citrus",
        ),
        (
            rsx! {
                Clapperboard {}
            },
            "Clapperboard",
        ),
        (
            rsx! {
                Clipboard {}
            },
            "Clipboard",
        ),
        (
            rsx! {
                ClipboardCheck {}
            },
            "Clipboard Check",
        ),
        (
            rsx! {
                ClipboardCopy {}
            },
            "Clipboard Copy",
        ),
        (
            rsx! {
                ClipboardList {}
            },
            "Clipboard List",
        ),
        (
            rsx! {
                ClipboardMinus {}
            },
            "Clipboard Minus",
        ),
        (
            rsx! {
                ClipboardPaste {}
            },
            "Clipboard Paste",
        ),
        (
            rsx! {
                ClipboardPen {}
            },
            "Clipboard Pen",
        ),
        (
            rsx! {
                ClipboardPenLine {}
            },
            "Clipboard Pen Line",
        ),
        (
            rsx! {
                ClipboardPlus {}
            },
            "Clipboard Plus",
        ),
        (
            rsx! {
                ClipboardType {}
            },
            "Clipboard Type",
        ),
        (
            rsx! {
                ClipboardX {}
            },
            "Clipboard X",
        ),
        (
            rsx! {
                Clock {}
            },
            "Clock",
        ),
        (
            rsx! {
                Clock1 {}
            },
            "Clock 1",
        ),
        (
            rsx! {
                Clock10 {}
            },
            "Clock 10",
        ),
        (
            rsx! {
                Clock11 {}
            },
            "Clock 11",
        ),
        (
            rsx! {
                Clock12 {}
            },
            "Clock 12",
        ),
        (
            rsx! {
                Clock2 {}
            },
            "Clock 2",
        ),
        (
            rsx! {
                Clock3 {}
            },
            "Clock 3",
        ),
        (
            rsx! {
                Clock4 {}
            },
            "Clock 4",
        ),
        (
            rsx! {
                Clock5 {}
            },
            "Clock 5",
        ),
        (
            rsx! {
                Clock6 {}
            },
            "Clock 6",
        ),
        (
            rsx! {
                Clock7 {}
            },
            "Clock 7",
        ),
        (
            rsx! {
                Clock8 {}
            },
            "Clock 8",
        ),
        (
            rsx! {
                Clock9 {}
            },
            "Clock 9",
        ),
        (
            rsx! {
                ClockAlert {}
            },
            "Clock Alert",
        ),
        (
            rsx! {
                ClockArrowDown {}
            },
            "Clock Arrow Down",
        ),
        (
            rsx! {
                ClockArrowUp {}
            },
            "Clock Arrow Up",
        ),
        (
            rsx! {
                ClockFading {}
            },
            "Clock Fading",
        ),
        (
            rsx! {
                ClockPlus {}
            },
            "Clock Plus",
        ),
        (
            rsx! {
                Cloud {}
            },
            "Cloud",
        ),
        (
            rsx! {
                CloudAlert {}
            },
            "Cloud Alert",
        ),
        (
            rsx! {
                CloudCog {}
            },
            "Cloud Cog",
        ),
        (
            rsx! {
                CloudDownload {}
            },
            "Cloud Download",
        ),
        (
            rsx! {
                CloudDrizzle {}
            },
            "Cloud Drizzle",
        ),
        (
            rsx! {
                CloudFog {}
            },
            "Cloud Fog",
        ),
        (
            rsx! {
                CloudHail {}
            },
            "Cloud Hail",
        ),
        (
            rsx! {
                CloudLightning {}
            },
            "Cloud Lightning",
        ),
        (
            rsx! {
                CloudMoon {}
            },
            "Cloud Moon",
        ),
        (
            rsx! {
                CloudMoonRain {}
            },
            "Cloud Moon Rain",
        ),
        (
            rsx! {
                CloudOff {}
            },
            "Cloud Off",
        ),
        (
            rsx! {
                CloudRain {}
            },
            "Cloud Rain",
        ),
        (
            rsx! {
                CloudRainWind {}
            },
            "Cloud Rain Wind",
        ),
        (
            rsx! {
                CloudSnow {}
            },
            "Cloud Snow",
        ),
        (
            rsx! {
                CloudSun {}
            },
            "Cloud Sun",
        ),
        (
            rsx! {
                CloudSunRain {}
            },
            "Cloud Sun Rain",
        ),
        (
            rsx! {
                CloudUpload {}
            },
            "Cloud Upload",
        ),
        (
            rsx! {
                Cloudy {}
            },
            "Cloudy",
        ),
        (
            rsx! {
                Clover {}
            },
            "Clover",
        ),
        (
            rsx! {
                Club {}
            },
            "Club",
        ),
        (
            rsx! {
                Code {}
            },
            "Code",
        ),
        (
            rsx! {
                CodeXml {}
            },
            "Code Xml",
        ),
        (
            rsx! {
                Codepen {}
            },
            "Codepen",
        ),
        (
            rsx! {
                Codesandbox {}
            },
            "Codesandbox",
        ),
        (
            rsx! {
                Coffee {}
            },
            "Coffee",
        ),
        (
            rsx! {
                Cog {}
            },
            "Cog",
        ),
        (
            rsx! {
                Coins {}
            },
            "Coins",
        ),
        (
            rsx! {
                Columns2 {}
            },
            "Columns 2",
        ),
        (
            rsx! {
                Columns3 {}
            },
            "Columns 3",
        ),
        (
            rsx! {
                Columns3Cog {}
            },
            "Columns 3 Cog",
        ),
        (
            rsx! {
                Columns4 {}
            },
            "Columns 4",
        ),
        (
            rsx! {
                Combine {}
            },
            "Combine",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsC3() -> Element {
    let icons = [
        (
            rsx! {
                Command {}
            },
            "Command",
        ),
        (
            rsx! {
                Compass {}
            },
            "Compass",
        ),
        (
            rsx! {
                Component {}
            },
            "Component",
        ),
        (
            rsx! {
                Computer {}
            },
            "Computer",
        ),
        (
            rsx! {
                ConciergeBell {}
            },
            "Concierge Bell",
        ),
        (
            rsx! {
                Cone {}
            },
            "Cone",
        ),
        (
            rsx! {
                Construction {}
            },
            "Construction",
        ),
        (
            rsx! {
                Contact {}
            },
            "Contact",
        ),
        (
            rsx! {
                ContactRound {}
            },
            "Contact Round",
        ),
        (
            rsx! {
                Container {}
            },
            "Container",
        ),
        (
            rsx! {
                Contrast {}
            },
            "Contrast",
        ),
        (
            rsx! {
                Cookie {}
            },
            "Cookie",
        ),
        (
            rsx! {
                CookingPot {}
            },
            "Cooking Pot",
        ),
        (
            rsx! {
                Copy {}
            },
            "Copy",
        ),
        (
            rsx! {
                CopyCheck {}
            },
            "Copy Check",
        ),
        (
            rsx! {
                CopyMinus {}
            },
            "Copy Minus",
        ),
        (
            rsx! {
                CopyPlus {}
            },
            "Copy Plus",
        ),
        (
            rsx! {
                CopySlash {}
            },
            "Copy Slash",
        ),
        (
            rsx! {
                CopyX {}
            },
            "Copy X",
        ),
        (
            rsx! {
                Copyleft {}
            },
            "Copyleft",
        ),
        (
            rsx! {
                Copyright {}
            },
            "Copyright",
        ),
        (
            rsx! {
                CornerDownLeft {}
            },
            "Corner Down Left",
        ),
        (
            rsx! {
                CornerDownRight {}
            },
            "Corner Down Right",
        ),
        (
            rsx! {
                CornerLeftDown {}
            },
            "Corner Left Down",
        ),
        (
            rsx! {
                CornerLeftUp {}
            },
            "Corner Left Up",
        ),
        (
            rsx! {
                CornerRightDown {}
            },
            "Corner Right Down",
        ),
        (
            rsx! {
                CornerRightUp {}
            },
            "Corner Right Up",
        ),
        (
            rsx! {
                CornerUpLeft {}
            },
            "Corner Up Left",
        ),
        (
            rsx! {
                CornerUpRight {}
            },
            "Corner Up Right",
        ),
        (
            rsx! {
                Cpu {}
            },
            "Cpu",
        ),
        (
            rsx! {
                CreativeCommons {}
            },
            "Creative Commons",
        ),
        (
            rsx! {
                CreditCard {}
            },
            "Credit Card",
        ),
        (
            rsx! {
                Croissant {}
            },
            "Croissant",
        ),
        (
            rsx! {
                Crop {}
            },
            "Crop",
        ),
        (
            rsx! {
                Cross {}
            },
            "Cross",
        ),
        (
            rsx! {
                Crosshair {}
            },
            "Crosshair",
        ),
        (
            rsx! {
                Crown {}
            },
            "Crown",
        ),
        (
            rsx! {
                Cuboid {}
            },
            "Cuboid",
        ),
        (
            rsx! {
                CupSoda {}
            },
            "Cup Soda",
        ),
        (
            rsx! {
                Currency {}
            },
            "Currency",
        ),
        (
            rsx! {
                Cylinder {}
            },
            "Cylinder",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsD1() -> Element {
    let icons = [
        (
            rsx! {
                Dam {}
            },
            "Dam",
        ),
        (
            rsx! {
                Database {}
            },
            "Database",
        ),
        (
            rsx! {
                DatabaseBackup {}
            },
            "Database Backup",
        ),
        (
            rsx! {
                DatabaseZap {}
            },
            "Database Zap",
        ),
        (
            rsx! {
                DecimalsArrowLeft {}
            },
            "Decimals Arrow Left",
        ),
        (
            rsx! {
                DecimalsArrowRight {}
            },
            "Decimals Arrow Right",
        ),
        (
            rsx! {
                Delete {}
            },
            "Delete",
        ),
        (
            rsx! {
                Dessert {}
            },
            "Dessert",
        ),
        (
            rsx! {
                Diameter {}
            },
            "Diameter",
        ),
        (
            rsx! {
                Diamond {}
            },
            "Diamond",
        ),
        (
            rsx! {
                DiamondMinus {}
            },
            "Diamond Minus",
        ),
        (
            rsx! {
                DiamondPercent {}
            },
            "Diamond Percent",
        ),
        (
            rsx! {
                DiamondPlus {}
            },
            "Diamond Plus",
        ),
        (
            rsx! {
                Dice1 {}
            },
            "Dice 1",
        ),
        (
            rsx! {
                Dice2 {}
            },
            "Dice 2",
        ),
        (
            rsx! {
                Dice3 {}
            },
            "Dice 3",
        ),
        (
            rsx! {
                Dice4 {}
            },
            "Dice 4",
        ),
        (
            rsx! {
                Dice5 {}
            },
            "Dice 5",
        ),
        (
            rsx! {
                Dice6 {}
            },
            "Dice 6",
        ),
        (
            rsx! {
                Dices {}
            },
            "Dices",
        ),
        (
            rsx! {
                Diff {}
            },
            "Diff",
        ),
        (
            rsx! {
                Disc {}
            },
            "Disc",
        ),
        (
            rsx! {
                Disc2 {}
            },
            "Disc 2",
        ),
        (
            rsx! {
                Disc3 {}
            },
            "Disc 3",
        ),
        (
            rsx! {
                DiscAlbum {}
            },
            "Disc Album",
        ),
        (
            rsx! {
                Divide {}
            },
            "Divide",
        ),
        (
            rsx! {
                Dna {}
            },
            "Dna",
        ),
        (
            rsx! {
                DnaOff {}
            },
            "Dna Off",
        ),
        (
            rsx! {
                Dock {}
            },
            "Dock",
        ),
        (
            rsx! {
                Dog {}
            },
            "Dog",
        ),
        (
            rsx! {
                DollarSign {}
            },
            "Dollar Sign",
        ),
        (
            rsx! {
                Donut {}
            },
            "Donut",
        ),
        (
            rsx! {
                DoorClosed {}
            },
            "Door Closed",
        ),
        (
            rsx! {
                DoorClosedLocked {}
            },
            "Door Closed Locked",
        ),
        (
            rsx! {
                DoorOpen {}
            },
            "Door Open",
        ),
        (
            rsx! {
                Dot {}
            },
            "Dot",
        ),
        (
            rsx! {
                Download {}
            },
            "Download",
        ),
        (
            rsx! {
                DraftingCompass {}
            },
            "Drafting Compass",
        ),
        (
            rsx! {
                Drama {}
            },
            "Drama",
        ),
        (
            rsx! {
                Dribbble {}
            },
            "Dribbble",
        ),
        (
            rsx! {
                Drill {}
            },
            "Drill",
        ),
        (
            rsx! {
                Droplet {}
            },
            "Droplet",
        ),
        (
            rsx! {
                DropletOff {}
            },
            "Droplet Off",
        ),
        (
            rsx! {
                Droplets {}
            },
            "Droplets",
        ),
        (
            rsx! {
                Drum {}
            },
            "Drum",
        ),
        (
            rsx! {
                Drumstick {}
            },
            "Drumstick",
        ),
        (
            rsx! {
                Dumbbell {}
            },
            "Dumbbell",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsE1() -> Element {
    let icons = [
        (
            rsx! {
                Ear {}
            },
            "Ear",
        ),
        (
            rsx! {
                EarOff {}
            },
            "Ear Off",
        ),
        (
            rsx! {
                Earth {}
            },
            "Earth",
        ),
        (
            rsx! {
                EarthLock {}
            },
            "Earth Lock",
        ),
        (
            rsx! {
                Eclipse {}
            },
            "Eclipse",
        ),
        (
            rsx! {
                Egg {}
            },
            "Egg",
        ),
        (
            rsx! {
                EggFried {}
            },
            "Egg Fried",
        ),
        (
            rsx! {
                EggOff {}
            },
            "Egg Off",
        ),
        (
            rsx! {
                Ellipsis {}
            },
            "Ellipsis",
        ),
        (
            rsx! {
                EllipsisVertical {}
            },
            "Ellipsis Vertical",
        ),
        (
            rsx! {
                Equal {}
            },
            "Equal",
        ),
        (
            rsx! {
                EqualApproximately {}
            },
            "Equal Approximately",
        ),
        (
            rsx! {
                EqualNot {}
            },
            "Equal Not",
        ),
        (
            rsx! {
                Eraser {}
            },
            "Eraser",
        ),
        (
            rsx! {
                EthernetPort {}
            },
            "Ethernet Port",
        ),
        (
            rsx! {
                Euro {}
            },
            "Euro",
        ),
        (
            rsx! {
                Expand {}
            },
            "Expand",
        ),
        (
            rsx! {
                ExternalLink {}
            },
            "External Link",
        ),
        (
            rsx! {
                Eye {}
            },
            "Eye",
        ),
        (
            rsx! {
                EyeClosed {}
            },
            "Eye Closed",
        ),
        (
            rsx! {
                EyeOff {}
            },
            "Eye Off",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsF1() -> Element {
    let icons = [
        (
            rsx! {
                Facebook {}
            },
            "Facebook",
        ),
        (
            rsx! {
                Factory {}
            },
            "Factory",
        ),
        (
            rsx! {
                Fan {}
            },
            "Fan",
        ),
        (
            rsx! {
                FastForward {}
            },
            "Fast Forward",
        ),
        (
            rsx! {
                Feather {}
            },
            "Feather",
        ),
        (
            rsx! {
                Fence {}
            },
            "Fence",
        ),
        (
            rsx! {
                FerrisWheel {}
            },
            "Ferris Wheel",
        ),
        (
            rsx! {
                Figma {}
            },
            "Figma",
        ),
        (
            rsx! {
                File {}
            },
            "File",
        ),
        (
            rsx! {
                FileArchive {}
            },
            "File Archive",
        ),
        (
            rsx! {
                FileAudio {}
            },
            "File Audio",
        ),
        (
            rsx! {
                FileAudio2 {}
            },
            "File Audio 2",
        ),
        (
            rsx! {
                FileAxis3D {}
            },
            "File Axis 3 D",
        ),
        (
            rsx! {
                FileBadge {}
            },
            "File Badge",
        ),
        (
            rsx! {
                FileBadge2 {}
            },
            "File Badge 2",
        ),
        (
            rsx! {
                FileBox {}
            },
            "File Box",
        ),
        (
            rsx! {
                FileChartColumn {}
            },
            "File Chart Column",
        ),
        (
            rsx! {
                FileChartColumnIncreasing {}
            },
            "File Chart Column Increasing",
        ),
        (
            rsx! {
                FileChartLine {}
            },
            "File Chart Line",
        ),
        (
            rsx! {
                FileChartPie {}
            },
            "File Chart Pie",
        ),
        (
            rsx! {
                FileCheck {}
            },
            "File Check",
        ),
        (
            rsx! {
                FileCheck2 {}
            },
            "File Check 2",
        ),
        (
            rsx! {
                FileClock {}
            },
            "File Clock",
        ),
        (
            rsx! {
                FileCode {}
            },
            "File Code",
        ),
        (
            rsx! {
                FileCode2 {}
            },
            "File Code 2",
        ),
        (
            rsx! {
                FileCog {}
            },
            "File Cog",
        ),
        (
            rsx! {
                FileDiff {}
            },
            "File Diff",
        ),
        (
            rsx! {
                FileDigit {}
            },
            "File Digit",
        ),
        (
            rsx! {
                FileDown {}
            },
            "File Down",
        ),
        (
            rsx! {
                FileHeart {}
            },
            "File Heart",
        ),
        (
            rsx! {
                FileImage {}
            },
            "File Image",
        ),
        (
            rsx! {
                FileInput {}
            },
            "File Input",
        ),
        (
            rsx! {
                FileJson {}
            },
            "File Json",
        ),
        (
            rsx! {
                FileJson2 {}
            },
            "File Json 2",
        ),
        (
            rsx! {
                FileKey {}
            },
            "File Key",
        ),
        (
            rsx! {
                FileKey2 {}
            },
            "File Key 2",
        ),
        (
            rsx! {
                FileLock {}
            },
            "File Lock",
        ),
        (
            rsx! {
                FileLock2 {}
            },
            "File Lock 2",
        ),
        (
            rsx! {
                FileMinus {}
            },
            "File Minus",
        ),
        (
            rsx! {
                FileMinus2 {}
            },
            "File Minus 2",
        ),
        (
            rsx! {
                FileMusic {}
            },
            "File Music",
        ),
        (
            rsx! {
                FileOutput {}
            },
            "File Output",
        ),
        (
            rsx! {
                FilePen {}
            },
            "File Pen",
        ),
        (
            rsx! {
                FilePenLine {}
            },
            "File Pen Line",
        ),
        (
            rsx! {
                FilePlus {}
            },
            "File Plus",
        ),
        (
            rsx! {
                FilePlus2 {}
            },
            "File Plus 2",
        ),
        (
            rsx! {
                FileQuestion {}
            },
            "File Question",
        ),
        (
            rsx! {
                FileScan {}
            },
            "File Scan",
        ),
        (
            rsx! {
                FileSearch {}
            },
            "File Search",
        ),
        (
            rsx! {
                FileSearch2 {}
            },
            "File Search 2",
        ),
        (
            rsx! {
                FileSliders {}
            },
            "File Sliders",
        ),
        (
            rsx! {
                FileSpreadsheet {}
            },
            "File Spreadsheet",
        ),
        (
            rsx! {
                FileStack {}
            },
            "File Stack",
        ),
        (
            rsx! {
                FileSymlink {}
            },
            "File Symlink",
        ),
        (
            rsx! {
                FileTerminal {}
            },
            "File Terminal",
        ),
        (
            rsx! {
                FileText {}
            },
            "File Text",
        ),
        (
            rsx! {
                FileType {}
            },
            "File Type",
        ),
        (
            rsx! {
                FileType2 {}
            },
            "File Type 2",
        ),
        (
            rsx! {
                FileUp {}
            },
            "File Up",
        ),
        (
            rsx! {
                FileUser {}
            },
            "File User",
        ),
        (
            rsx! {
                FileVideo {}
            },
            "File Video",
        ),
        (
            rsx! {
                FileVideo2 {}
            },
            "File Video 2",
        ),
        (
            rsx! {
                FileVolume {}
            },
            "File Volume",
        ),
        (
            rsx! {
                FileVolume2 {}
            },
            "File Volume 2",
        ),
        (
            rsx! {
                FileWarning {}
            },
            "File Warning",
        ),
        (
            rsx! {
                FileX {}
            },
            "File X",
        ),
        (
            rsx! {
                FileX2 {}
            },
            "File X 2",
        ),
        (
            rsx! {
                Files {}
            },
            "Files",
        ),
        (
            rsx! {
                Film {}
            },
            "Film",
        ),
        (
            rsx! {
                Fingerprint {}
            },
            "Fingerprint",
        ),
        (
            rsx! {
                FireExtinguisher {}
            },
            "Fire Extinguisher",
        ),
        (
            rsx! {
                Fish {}
            },
            "Fish",
        ),
        (
            rsx! {
                FishOff {}
            },
            "Fish Off",
        ),
        (
            rsx! {
                FishSymbol {}
            },
            "Fish Symbol",
        ),
        (
            rsx! {
                Flag {}
            },
            "Flag",
        ),
        (
            rsx! {
                FlagOff {}
            },
            "Flag Off",
        ),
        (
            rsx! {
                FlagTriangleLeft {}
            },
            "Flag Triangle Left",
        ),
        (
            rsx! {
                FlagTriangleRight {}
            },
            "Flag Triangle Right",
        ),
        (
            rsx! {
                Flame {}
            },
            "Flame",
        ),
        (
            rsx! {
                FlameKindling {}
            },
            "Flame Kindling",
        ),
        (
            rsx! {
                Flashlight {}
            },
            "Flashlight",
        ),
        (
            rsx! {
                FlashlightOff {}
            },
            "Flashlight Off",
        ),
        (
            rsx! {
                FlaskConical {}
            },
            "Flask Conical",
        ),
        (
            rsx! {
                FlaskConicalOff {}
            },
            "Flask Conical Off",
        ),
        (
            rsx! {
                FlaskRound {}
            },
            "Flask Round",
        ),
        (
            rsx! {
                FlipHorizontal {}
            },
            "Flip Horizontal",
        ),
        (
            rsx! {
                FlipHorizontal2 {}
            },
            "Flip Horizontal 2",
        ),
        (
            rsx! {
                FlipVertical {}
            },
            "Flip Vertical",
        ),
        (
            rsx! {
                FlipVertical2 {}
            },
            "Flip Vertical 2",
        ),
        (
            rsx! {
                Flower {}
            },
            "Flower",
        ),
        (
            rsx! {
                Flower2 {}
            },
            "Flower 2",
        ),
        (
            rsx! {
                Focus {}
            },
            "Focus",
        ),
        (
            rsx! {
                FoldHorizontal {}
            },
            "Fold Horizontal",
        ),
        (
            rsx! {
                FoldVertical {}
            },
            "Fold Vertical",
        ),
        (
            rsx! {
                Folder {}
            },
            "Folder",
        ),
        (
            rsx! {
                FolderArchive {}
            },
            "Folder Archive",
        ),
        (
            rsx! {
                FolderCheck {}
            },
            "Folder Check",
        ),
        (
            rsx! {
                FolderClock {}
            },
            "Folder Clock",
        ),
        (
            rsx! {
                FolderClosed {}
            },
            "Folder Closed",
        ),
        (
            rsx! {
                FolderCode {}
            },
            "Folder Code",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsF2() -> Element {
    let icons = [
        (
            rsx! {
                FolderCog {}
            },
            "Folder Cog",
        ),
        (
            rsx! {
                FolderDot {}
            },
            "Folder Dot",
        ),
        (
            rsx! {
                FolderDown {}
            },
            "Folder Down",
        ),
        (
            rsx! {
                FolderGit {}
            },
            "Folder Git",
        ),
        (
            rsx! {
                FolderGit2 {}
            },
            "Folder Git 2",
        ),
        (
            rsx! {
                FolderHeart {}
            },
            "Folder Heart",
        ),
        (
            rsx! {
                FolderInput {}
            },
            "Folder Input",
        ),
        (
            rsx! {
                FolderKanban {}
            },
            "Folder Kanban",
        ),
        (
            rsx! {
                FolderKey {}
            },
            "Folder Key",
        ),
        (
            rsx! {
                FolderLock {}
            },
            "Folder Lock",
        ),
        (
            rsx! {
                FolderMinus {}
            },
            "Folder Minus",
        ),
        (
            rsx! {
                FolderOpen {}
            },
            "Folder Open",
        ),
        (
            rsx! {
                FolderOpenDot {}
            },
            "Folder Open Dot",
        ),
        (
            rsx! {
                FolderOutput {}
            },
            "Folder Output",
        ),
        (
            rsx! {
                FolderPen {}
            },
            "Folder Pen",
        ),
        (
            rsx! {
                FolderPlus {}
            },
            "Folder Plus",
        ),
        (
            rsx! {
                FolderRoot {}
            },
            "Folder Root",
        ),
        (
            rsx! {
                FolderSearch {}
            },
            "Folder Search",
        ),
        (
            rsx! {
                FolderSearch2 {}
            },
            "Folder Search 2",
        ),
        (
            rsx! {
                FolderSymlink {}
            },
            "Folder Symlink",
        ),
        (
            rsx! {
                FolderSync {}
            },
            "Folder Sync",
        ),
        (
            rsx! {
                FolderTree {}
            },
            "Folder Tree",
        ),
        (
            rsx! {
                FolderUp {}
            },
            "Folder Up",
        ),
        (
            rsx! {
                FolderX {}
            },
            "Folder X",
        ),
        (
            rsx! {
                Folders {}
            },
            "Folders",
        ),
        (
            rsx! {
                Footprints {}
            },
            "Footprints",
        ),
        (
            rsx! {
                Forklift {}
            },
            "Forklift",
        ),
        (
            rsx! {
                Forward {}
            },
            "Forward",
        ),
        (
            rsx! {
                Frame {}
            },
            "Frame",
        ),
        (
            rsx! {
                Framer {}
            },
            "Framer",
        ),
        (
            rsx! {
                Frown {}
            },
            "Frown",
        ),
        (
            rsx! {
                Fuel {}
            },
            "Fuel",
        ),
        (
            rsx! {
                Fullscreen {}
            },
            "Fullscreen",
        ),
        (
            rsx! {
                Funnel {}
            },
            "Funnel",
        ),
        (
            rsx! {
                FunnelPlus {}
            },
            "Funnel Plus",
        ),
        (
            rsx! {
                FunnelX {}
            },
            "Funnel X",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsG1() -> Element {
    let icons = [
        (
            rsx! {
                GalleryHorizontal {}
            },
            "Gallery Horizontal",
        ),
        (
            rsx! {
                GalleryHorizontalEnd {}
            },
            "Gallery Horizontal End",
        ),
        (
            rsx! {
                GalleryThumbnails {}
            },
            "Gallery Thumbnails",
        ),
        (
            rsx! {
                GalleryVertical {}
            },
            "Gallery Vertical",
        ),
        (
            rsx! {
                GalleryVerticalEnd {}
            },
            "Gallery Vertical End",
        ),
        (
            rsx! {
                Gamepad {}
            },
            "Gamepad",
        ),
        (
            rsx! {
                Gamepad2 {}
            },
            "Gamepad 2",
        ),
        (
            rsx! {
                Gauge {}
            },
            "Gauge",
        ),
        (
            rsx! {
                Gavel {}
            },
            "Gavel",
        ),
        (
            rsx! {
                Gem {}
            },
            "Gem",
        ),
        (
            rsx! {
                Ghost {}
            },
            "Ghost",
        ),
        (
            rsx! {
                Gift {}
            },
            "Gift",
        ),
        (
            rsx! {
                GitBranch {}
            },
            "Git Branch",
        ),
        (
            rsx! {
                GitBranchPlus {}
            },
            "Git Branch Plus",
        ),
        (
            rsx! {
                GitCommitHorizontal {}
            },
            "Git Commit Horizontal",
        ),
        (
            rsx! {
                GitCommitVertical {}
            },
            "Git Commit Vertical",
        ),
        (
            rsx! {
                GitCompare {}
            },
            "Git Compare",
        ),
        (
            rsx! {
                GitCompareArrows {}
            },
            "Git Compare Arrows",
        ),
        (
            rsx! {
                GitFork {}
            },
            "Git Fork",
        ),
        (
            rsx! {
                GitGraph {}
            },
            "Git Graph",
        ),
        (
            rsx! {
                GitMerge {}
            },
            "Git Merge",
        ),
        (
            rsx! {
                GitPullRequest {}
            },
            "Git Pull Request",
        ),
        (
            rsx! {
                GitPullRequestArrow {}
            },
            "Git Pull Request Arrow",
        ),
        (
            rsx! {
                GitPullRequestClosed {}
            },
            "Git Pull Request Closed",
        ),
        (
            rsx! {
                GitPullRequestCreate {}
            },
            "Git Pull Request Create",
        ),
        (
            rsx! {
                GitPullRequestCreateArrow {}
            },
            "Git Pull Request Create Arrow",
        ),
        (
            rsx! {
                GitPullRequestDraft {}
            },
            "Git Pull Request Draft",
        ),
        (
            rsx! {
                Github {}
            },
            "Github",
        ),
        (
            rsx! {
                Gitlab {}
            },
            "Gitlab",
        ),
        (
            rsx! {
                GlassWater {}
            },
            "Glass Water",
        ),
        (
            rsx! {
                Glasses {}
            },
            "Glasses",
        ),
        (
            rsx! {
                Globe {}
            },
            "Globe",
        ),
        (
            rsx! {
                GlobeLock {}
            },
            "Globe Lock",
        ),
        (
            rsx! {
                Goal {}
            },
            "Goal",
        ),
        (
            rsx! {
                Gpu {}
            },
            "Gpu",
        ),
        (
            rsx! {
                Grab {}
            },
            "Grab",
        ),
        (
            rsx! {
                GraduationCap {}
            },
            "Graduation Cap",
        ),
        (
            rsx! {
                Grape {}
            },
            "Grape",
        ),
        (
            rsx! {
                Grid2X2 {}
            },
            "Grid 2 X 2",
        ),
        (
            rsx! {
                Grid2X2Check {}
            },
            "Grid 2 X 2 Check",
        ),
        (
            rsx! {
                Grid2X2Plus {}
            },
            "Grid 2 X 2 Plus",
        ),
        (
            rsx! {
                Grid2X2X {}
            },
            "Grid 2 X 2 X",
        ),
        (
            rsx! {
                Grid3X3 {}
            },
            "Grid 3 X 3",
        ),
        (
            rsx! {
                Grip {}
            },
            "Grip",
        ),
        (
            rsx! {
                GripHorizontal {}
            },
            "Grip Horizontal",
        ),
        (
            rsx! {
                GripVertical {}
            },
            "Grip Vertical",
        ),
        (
            rsx! {
                Group {}
            },
            "Group",
        ),
        (
            rsx! {
                Guitar {}
            },
            "Guitar",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsH1() -> Element {
    let icons = [
        (
            rsx! {
                Ham {}
            },
            "Ham",
        ),
        (
            rsx! {
                Hamburger {}
            },
            "Hamburger",
        ),
        (
            rsx! {
                Hammer {}
            },
            "Hammer",
        ),
        (
            rsx! {
                Hand {}
            },
            "Hand",
        ),
        (
            rsx! {
                HandCoins {}
            },
            "Hand Coins",
        ),
        (
            rsx! {
                HandHeart {}
            },
            "Hand Heart",
        ),
        (
            rsx! {
                HandHelping {}
            },
            "Hand Helping",
        ),
        (
            rsx! {
                HandMetal {}
            },
            "Hand Metal",
        ),
        (
            rsx! {
                HandPlatter {}
            },
            "Hand Platter",
        ),
        (
            rsx! {
                Handshake {}
            },
            "Handshake",
        ),
        (
            rsx! {
                HardDrive {}
            },
            "Hard Drive",
        ),
        (
            rsx! {
                HardDriveDownload {}
            },
            "Hard Drive Download",
        ),
        (
            rsx! {
                HardDriveUpload {}
            },
            "Hard Drive Upload",
        ),
        (
            rsx! {
                HardHat {}
            },
            "Hard Hat",
        ),
        (
            rsx! {
                Hash {}
            },
            "Hash",
        ),
        (
            rsx! {
                Haze {}
            },
            "Haze",
        ),
        (
            rsx! {
                HdmiPort {}
            },
            "Hdmi Port",
        ),
        (
            rsx! {
                Heading {}
            },
            "Heading",
        ),
        (
            rsx! {
                Heading1 {}
            },
            "Heading 1",
        ),
        (
            rsx! {
                Heading2 {}
            },
            "Heading 2",
        ),
        (
            rsx! {
                Heading3 {}
            },
            "Heading 3",
        ),
        (
            rsx! {
                Heading4 {}
            },
            "Heading 4",
        ),
        (
            rsx! {
                Heading5 {}
            },
            "Heading 5",
        ),
        (
            rsx! {
                Heading6 {}
            },
            "Heading 6",
        ),
        (
            rsx! {
                HeadphoneOff {}
            },
            "Headphone Off",
        ),
        (
            rsx! {
                Headphones {}
            },
            "Headphones",
        ),
        (
            rsx! {
                Headset {}
            },
            "Headset",
        ),
        (
            rsx! {
                Heart {}
            },
            "Heart",
        ),
        (
            rsx! {
                HeartCrack {}
            },
            "Heart Crack",
        ),
        (
            rsx! {
                HeartHandshake {}
            },
            "Heart Handshake",
        ),
        (
            rsx! {
                HeartMinus {}
            },
            "Heart Minus",
        ),
        (
            rsx! {
                HeartOff {}
            },
            "Heart Off",
        ),
        (
            rsx! {
                HeartPlus {}
            },
            "Heart Plus",
        ),
        (
            rsx! {
                HeartPulse {}
            },
            "Heart Pulse",
        ),
        (
            rsx! {
                Heater {}
            },
            "Heater",
        ),
        (
            rsx! {
                Hexagon {}
            },
            "Hexagon",
        ),
        (
            rsx! {
                Highlighter {}
            },
            "Highlighter",
        ),
        (
            rsx! {
                History {}
            },
            "History",
        ),
        (
            rsx! {
                Hop {}
            },
            "Hop",
        ),
        (
            rsx! {
                HopOff {}
            },
            "Hop Off",
        ),
        (
            rsx! {
                Hospital {}
            },
            "Hospital",
        ),
        (
            rsx! {
                Hotel {}
            },
            "Hotel",
        ),
        (
            rsx! {
                Hourglass {}
            },
            "Hourglass",
        ),
        (
            rsx! {
                House {}
            },
            "House",
        ),
        (
            rsx! {
                HousePlug {}
            },
            "House Plug",
        ),
        (
            rsx! {
                HousePlus {}
            },
            "House Plus",
        ),
        (
            rsx! {
                HouseWifi {}
            },
            "House Wifi",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsI1() -> Element {
    let icons = [
        (
            rsx! {
                IceCreamBowl {}
            },
            "Ice Cream Bowl",
        ),
        (
            rsx! {
                IceCreamCone {}
            },
            "Ice Cream Cone",
        ),
        (
            rsx! {
                IdCard {}
            },
            "Id Card",
        ),
        (
            rsx! {
                Image {}
            },
            "Image",
        ),
        (
            rsx! {
                ImageDown {}
            },
            "Image Down",
        ),
        (
            rsx! {
                ImageMinus {}
            },
            "Image Minus",
        ),
        (
            rsx! {
                ImageOff {}
            },
            "Image Off",
        ),
        (
            rsx! {
                ImagePlay {}
            },
            "Image Play",
        ),
        (
            rsx! {
                ImagePlus {}
            },
            "Image Plus",
        ),
        (
            rsx! {
                ImageUp {}
            },
            "Image Up",
        ),
        (
            rsx! {
                ImageUpscale {}
            },
            "Image Upscale",
        ),
        (
            rsx! {
                Images {}
            },
            "Images",
        ),
        (
            rsx! {
                Import {}
            },
            "Import",
        ),
        (
            rsx! {
                Inbox {}
            },
            "Inbox",
        ),
        (
            rsx! {
                IndentDecrease {}
            },
            "Indent Decrease",
        ),
        (
            rsx! {
                IndentIncrease {}
            },
            "Indent Increase",
        ),
        (
            rsx! {
                IndianRupee {}
            },
            "Indian Rupee",
        ),
        (
            rsx! {
                Infinity {}
            },
            "Infinity",
        ),
        (
            rsx! {
                Info {}
            },
            "Info",
        ),
        (
            rsx! {
                InspectionPanel {}
            },
            "Inspection Panel",
        ),
        (
            rsx! {
                Instagram {}
            },
            "Instagram",
        ),
        (
            rsx! {
                Italic {}
            },
            "Italic",
        ),
        (
            rsx! {
                IterationCcw {}
            },
            "Iteration Ccw",
        ),
        (
            rsx! {
                IterationCw {}
            },
            "Iteration Cw",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsJ1() -> Element {
    let icons = [
        (
            rsx! {
                JapaneseYen {}
            },
            "Japanese Yen",
        ),
        (
            rsx! {
                Joystick {}
            },
            "Joystick",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsK1() -> Element {
    let icons = [
        (
            rsx! {
                Kanban {}
            },
            "Kanban",
        ),
        (
            rsx! {
                Key {}
            },
            "Key",
        ),
        (
            rsx! {
                KeyRound {}
            },
            "Key Round",
        ),
        (
            rsx! {
                KeySquare {}
            },
            "Key Square",
        ),
        (
            rsx! {
                Keyboard {}
            },
            "Keyboard",
        ),
        (
            rsx! {
                KeyboardMusic {}
            },
            "Keyboard Music",
        ),
        (
            rsx! {
                KeyboardOff {}
            },
            "Keyboard Off",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsL1() -> Element {
    let icons = [
        (
            rsx! {
                Lamp {}
            },
            "Lamp",
        ),
        (
            rsx! {
                LampCeiling {}
            },
            "Lamp Ceiling",
        ),
        (
            rsx! {
                LampDesk {}
            },
            "Lamp Desk",
        ),
        (
            rsx! {
                LampFloor {}
            },
            "Lamp Floor",
        ),
        (
            rsx! {
                LampWallDown {}
            },
            "Lamp Wall Down",
        ),
        (
            rsx! {
                LampWallUp {}
            },
            "Lamp Wall Up",
        ),
        (
            rsx! {
                LandPlot {}
            },
            "Land Plot",
        ),
        (
            rsx! {
                Landmark {}
            },
            "Landmark",
        ),
        (
            rsx! {
                Languages {}
            },
            "Languages",
        ),
        (
            rsx! {
                Laptop {}
            },
            "Laptop",
        ),
        (
            rsx! {
                LaptopMinimal {}
            },
            "Laptop Minimal",
        ),
        (
            rsx! {
                LaptopMinimalCheck {}
            },
            "Laptop Minimal Check",
        ),
        (
            rsx! {
                Lasso {}
            },
            "Lasso",
        ),
        (
            rsx! {
                LassoSelect {}
            },
            "Lasso Select",
        ),
        (
            rsx! {
                Laugh {}
            },
            "Laugh",
        ),
        (
            rsx! {
                Layers {}
            },
            "Layers",
        ),
        (
            rsx! {
                Layers2 {}
            },
            "Layers 2",
        ),
        (
            rsx! {
                LayoutDashboard {}
            },
            "Layout Dashboard",
        ),
        (
            rsx! {
                LayoutGrid {}
            },
            "Layout Grid",
        ),
        (
            rsx! {
                LayoutList {}
            },
            "Layout List",
        ),
        (
            rsx! {
                LayoutPanelLeft {}
            },
            "Layout Panel Left",
        ),
        (
            rsx! {
                LayoutPanelTop {}
            },
            "Layout Panel Top",
        ),
        (
            rsx! {
                LayoutTemplate {}
            },
            "Layout Template",
        ),
        (
            rsx! {
                Leaf {}
            },
            "Leaf",
        ),
        (
            rsx! {
                LeafyGreen {}
            },
            "Leafy Green",
        ),
        (
            rsx! {
                Lectern {}
            },
            "Lectern",
        ),
        (
            rsx! {
                LetterText {}
            },
            "Letter Text",
        ),
        (
            rsx! {
                Library {}
            },
            "Library",
        ),
        (
            rsx! {
                LibraryBig {}
            },
            "Library Big",
        ),
        (
            rsx! {
                LifeBuoy {}
            },
            "Life Buoy",
        ),
        (
            rsx! {
                Ligature {}
            },
            "Ligature",
        ),
        (
            rsx! {
                Lightbulb {}
            },
            "Lightbulb",
        ),
        (
            rsx! {
                LightbulbOff {}
            },
            "Lightbulb Off",
        ),
        (
            rsx! {
                Link {}
            },
            "Link",
        ),
        (
            rsx! {
                Link2 {}
            },
            "Link 2",
        ),
        (
            rsx! {
                Link2Off {}
            },
            "Link 2 Off",
        ),
        (
            rsx! {
                Linkedin {}
            },
            "Linkedin",
        ),
        (
            rsx! {
                List {}
            },
            "List",
        ),
        (
            rsx! {
                ListCheck {}
            },
            "List Check",
        ),
        (
            rsx! {
                ListChecks {}
            },
            "List Checks",
        ),
        (
            rsx! {
                ListCollapse {}
            },
            "List Collapse",
        ),
        (
            rsx! {
                ListEnd {}
            },
            "List End",
        ),
        (
            rsx! {
                ListFilter {}
            },
            "List Filter",
        ),
        (
            rsx! {
                ListFilterPlus {}
            },
            "List Filter Plus",
        ),
        (
            rsx! {
                ListMinus {}
            },
            "List Minus",
        ),
        (
            rsx! {
                ListMusic {}
            },
            "List Music",
        ),
        (
            rsx! {
                ListOrdered {}
            },
            "List Ordered",
        ),
        (
            rsx! {
                ListPlus {}
            },
            "List Plus",
        ),
        (
            rsx! {
                ListRestart {}
            },
            "List Restart",
        ),
        (
            rsx! {
                ListStart {}
            },
            "List Start",
        ),
        (
            rsx! {
                ListTodo {}
            },
            "List Todo",
        ),
        (
            rsx! {
                ListTree {}
            },
            "List Tree",
        ),
        (
            rsx! {
                ListVideo {}
            },
            "List Video",
        ),
        (
            rsx! {
                ListX {}
            },
            "List X",
        ),
        (
            rsx! {
                Loader {}
            },
            "Loader",
        ),
        (
            rsx! {
                LoaderCircle {}
            },
            "Loader Circle",
        ),
        (
            rsx! {
                LoaderPinwheel {}
            },
            "Loader Pinwheel",
        ),
        (
            rsx! {
                Locate {}
            },
            "Locate",
        ),
        (
            rsx! {
                LocateFixed {}
            },
            "Locate Fixed",
        ),
        (
            rsx! {
                LocateOff {}
            },
            "Locate Off",
        ),
        (
            rsx! {
                LocationEdit {}
            },
            "Location Edit",
        ),
        (
            rsx! {
                Lock {}
            },
            "Lock",
        ),
        (
            rsx! {
                LockKeyhole {}
            },
            "Lock Keyhole",
        ),
        (
            rsx! {
                LockKeyholeOpen {}
            },
            "Lock Keyhole Open",
        ),
        (
            rsx! {
                LockOpen {}
            },
            "Lock Open",
        ),
        (
            rsx! {
                LogIn {}
            },
            "Log In",
        ),
        (
            rsx! {
                LogOut {}
            },
            "Log Out",
        ),
        (
            rsx! {
                Logs {}
            },
            "Logs",
        ),
        (
            rsx! {
                Lollipop {}
            },
            "Lollipop",
        ),
        (
            rsx! {
                Luggage {}
            },
            "Luggage",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsM1() -> Element {
    let icons = [
        (
            rsx! {
                Magnet {}
            },
            "Magnet",
        ),
        (
            rsx! {
                Mail {}
            },
            "Mail",
        ),
        (
            rsx! {
                MailCheck {}
            },
            "Mail Check",
        ),
        (
            rsx! {
                MailMinus {}
            },
            "Mail Minus",
        ),
        (
            rsx! {
                MailOpen {}
            },
            "Mail Open",
        ),
        (
            rsx! {
                MailPlus {}
            },
            "Mail Plus",
        ),
        (
            rsx! {
                MailQuestion {}
            },
            "Mail Question",
        ),
        (
            rsx! {
                MailSearch {}
            },
            "Mail Search",
        ),
        (
            rsx! {
                MailWarning {}
            },
            "Mail Warning",
        ),
        (
            rsx! {
                MailX {}
            },
            "Mail X",
        ),
        (
            rsx! {
                Mailbox {}
            },
            "Mailbox",
        ),
        (
            rsx! {
                Mails {}
            },
            "Mails",
        ),
        (
            rsx! {
                Map {}
            },
            "Map",
        ),
        (
            rsx! {
                MapPin {}
            },
            "Map Pin",
        ),
        (
            rsx! {
                MapPinCheck {}
            },
            "Map Pin Check",
        ),
        (
            rsx! {
                MapPinCheckInside {}
            },
            "Map Pin Check Inside",
        ),
        (
            rsx! {
                MapPinHouse {}
            },
            "Map Pin House",
        ),
        (
            rsx! {
                MapPinMinus {}
            },
            "Map Pin Minus",
        ),
        (
            rsx! {
                MapPinMinusInside {}
            },
            "Map Pin Minus Inside",
        ),
        (
            rsx! {
                MapPinOff {}
            },
            "Map Pin Off",
        ),
        (
            rsx! {
                MapPinPlus {}
            },
            "Map Pin Plus",
        ),
        (
            rsx! {
                MapPinPlusInside {}
            },
            "Map Pin Plus Inside",
        ),
        (
            rsx! {
                MapPinX {}
            },
            "Map Pin X",
        ),
        (
            rsx! {
                MapPinXInside {}
            },
            "Map Pin X Inside",
        ),
        (
            rsx! {
                MapPinned {}
            },
            "Map Pinned",
        ),
        (
            rsx! {
                MapPlus {}
            },
            "Map Plus",
        ),
        (
            rsx! {
                Mars {}
            },
            "Mars",
        ),
        (
            rsx! {
                MarsStroke {}
            },
            "Mars Stroke",
        ),
        (
            rsx! {
                Martini {}
            },
            "Martini",
        ),
        (
            rsx! {
                Maximize {}
            },
            "Maximize",
        ),
        (
            rsx! {
                Maximize2 {}
            },
            "Maximize 2",
        ),
        (
            rsx! {
                Medal {}
            },
            "Medal",
        ),
        (
            rsx! {
                Megaphone {}
            },
            "Megaphone",
        ),
        (
            rsx! {
                MegaphoneOff {}
            },
            "Megaphone Off",
        ),
        (
            rsx! {
                Meh {}
            },
            "Meh",
        ),
        (
            rsx! {
                MemoryStick {}
            },
            "Memory Stick",
        ),
        (
            rsx! {
                Menu {}
            },
            "Menu",
        ),
        (
            rsx! {
                Merge {}
            },
            "Merge",
        ),
        (
            rsx! {
                MessageCircle {}
            },
            "Message Circle",
        ),
        (
            rsx! {
                MessageCircleCode {}
            },
            "Message Circle Code",
        ),
        (
            rsx! {
                MessageCircleDashed {}
            },
            "Message Circle Dashed",
        ),
        (
            rsx! {
                MessageCircleHeart {}
            },
            "Message Circle Heart",
        ),
        (
            rsx! {
                MessageCircleMore {}
            },
            "Message Circle More",
        ),
        (
            rsx! {
                MessageCircleOff {}
            },
            "Message Circle Off",
        ),
        (
            rsx! {
                MessageCirclePlus {}
            },
            "Message Circle Plus",
        ),
        (
            rsx! {
                MessageCircleQuestion {}
            },
            "Message Circle Question",
        ),
        (
            rsx! {
                MessageCircleReply {}
            },
            "Message Circle Reply",
        ),
        (
            rsx! {
                MessageCircleWarning {}
            },
            "Message Circle Warning",
        ),
        (
            rsx! {
                MessageCircleX {}
            },
            "Message Circle X",
        ),
        (
            rsx! {
                MessageSquare {}
            },
            "Message Square",
        ),
        (
            rsx! {
                MessageSquareCode {}
            },
            "Message Square Code",
        ),
        (
            rsx! {
                MessageSquareDashed {}
            },
            "Message Square Dashed",
        ),
        (
            rsx! {
                MessageSquareDiff {}
            },
            "Message Square Diff",
        ),
        (
            rsx! {
                MessageSquareDot {}
            },
            "Message Square Dot",
        ),
        (
            rsx! {
                MessageSquareHeart {}
            },
            "Message Square Heart",
        ),
        (
            rsx! {
                MessageSquareLock {}
            },
            "Message Square Lock",
        ),
        (
            rsx! {
                MessageSquareMore {}
            },
            "Message Square More",
        ),
        (
            rsx! {
                MessageSquareOff {}
            },
            "Message Square Off",
        ),
        (
            rsx! {
                MessageSquarePlus {}
            },
            "Message Square Plus",
        ),
        (
            rsx! {
                MessageSquareQuote {}
            },
            "Message Square Quote",
        ),
        (
            rsx! {
                MessageSquareReply {}
            },
            "Message Square Reply",
        ),
        (
            rsx! {
                MessageSquareShare {}
            },
            "Message Square Share",
        ),
        (
            rsx! {
                MessageSquareText {}
            },
            "Message Square Text",
        ),
        (
            rsx! {
                MessageSquareWarning {}
            },
            "Message Square Warning",
        ),
        (
            rsx! {
                MessageSquareX {}
            },
            "Message Square X",
        ),
        (
            rsx! {
                MessagesSquare {}
            },
            "Messages Square",
        ),
        (
            rsx! {
                Mic {}
            },
            "Mic",
        ),
        (
            rsx! {
                MicOff {}
            },
            "Mic Off",
        ),
        (
            rsx! {
                MicVocal {}
            },
            "Mic Vocal",
        ),
        (
            rsx! {
                Microchip {}
            },
            "Microchip",
        ),
        (
            rsx! {
                Microscope {}
            },
            "Microscope",
        ),
        (
            rsx! {
                Microwave {}
            },
            "Microwave",
        ),
        (
            rsx! {
                Milestone {}
            },
            "Milestone",
        ),
        (
            rsx! {
                Milk {}
            },
            "Milk",
        ),
        (
            rsx! {
                MilkOff {}
            },
            "Milk Off",
        ),
        (
            rsx! {
                Minimize {}
            },
            "Minimize",
        ),
        (
            rsx! {
                Minimize2 {}
            },
            "Minimize 2",
        ),
        (
            rsx! {
                Minus {}
            },
            "Minus",
        ),
        (
            rsx! {
                Monitor {}
            },
            "Monitor",
        ),
        (
            rsx! {
                MonitorCheck {}
            },
            "Monitor Check",
        ),
        (
            rsx! {
                MonitorCog {}
            },
            "Monitor Cog",
        ),
        (
            rsx! {
                MonitorDot {}
            },
            "Monitor Dot",
        ),
        (
            rsx! {
                MonitorDown {}
            },
            "Monitor Down",
        ),
        (
            rsx! {
                MonitorOff {}
            },
            "Monitor Off",
        ),
        (
            rsx! {
                MonitorPause {}
            },
            "Monitor Pause",
        ),
        (
            rsx! {
                MonitorPlay {}
            },
            "Monitor Play",
        ),
        (
            rsx! {
                MonitorSmartphone {}
            },
            "Monitor Smartphone",
        ),
        (
            rsx! {
                MonitorSpeaker {}
            },
            "Monitor Speaker",
        ),
        (
            rsx! {
                MonitorStop {}
            },
            "Monitor Stop",
        ),
        (
            rsx! {
                MonitorUp {}
            },
            "Monitor Up",
        ),
        (
            rsx! {
                MonitorX {}
            },
            "Monitor X",
        ),
        (
            rsx! {
                Moon {}
            },
            "Moon",
        ),
        (
            rsx! {
                MoonStar {}
            },
            "Moon Star",
        ),
        (
            rsx! {
                Mountain {}
            },
            "Mountain",
        ),
        (
            rsx! {
                MountainSnow {}
            },
            "Mountain Snow",
        ),
        (
            rsx! {
                Mouse {}
            },
            "Mouse",
        ),
        (
            rsx! {
                MouseOff {}
            },
            "Mouse Off",
        ),
        (
            rsx! {
                MousePointer {}
            },
            "Mouse Pointer",
        ),
        (
            rsx! {
                MousePointer2 {}
            },
            "Mouse Pointer 2",
        ),
        (
            rsx! {
                MousePointerBan {}
            },
            "Mouse Pointer Ban",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsM2() -> Element {
    let icons = [
        (
            rsx! {
                MousePointerClick {}
            },
            "Mouse Pointer Click",
        ),
        (
            rsx! {
                Move {}
            },
            "Move",
        ),
        (
            rsx! {
                Move3D {}
            },
            "Move 3 D",
        ),
        (
            rsx! {
                MoveDiagonal {}
            },
            "Move Diagonal",
        ),
        (
            rsx! {
                MoveDiagonal2 {}
            },
            "Move Diagonal 2",
        ),
        (
            rsx! {
                MoveDown {}
            },
            "Move Down",
        ),
        (
            rsx! {
                MoveDownLeft {}
            },
            "Move Down Left",
        ),
        (
            rsx! {
                MoveDownRight {}
            },
            "Move Down Right",
        ),
        (
            rsx! {
                MoveHorizontal {}
            },
            "Move Horizontal",
        ),
        (
            rsx! {
                MoveLeft {}
            },
            "Move Left",
        ),
        (
            rsx! {
                MoveRight {}
            },
            "Move Right",
        ),
        (
            rsx! {
                MoveUp {}
            },
            "Move Up",
        ),
        (
            rsx! {
                MoveUpLeft {}
            },
            "Move Up Left",
        ),
        (
            rsx! {
                MoveUpRight {}
            },
            "Move Up Right",
        ),
        (
            rsx! {
                MoveVertical {}
            },
            "Move Vertical",
        ),
        (
            rsx! {
                Music {}
            },
            "Music",
        ),
        (
            rsx! {
                Music2 {}
            },
            "Music 2",
        ),
        (
            rsx! {
                Music3 {}
            },
            "Music 3",
        ),
        (
            rsx! {
                Music4 {}
            },
            "Music 4",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsN1() -> Element {
    let icons = [
        (
            rsx! {
                Navigation {}
            },
            "Navigation",
        ),
        (
            rsx! {
                Navigation2 {}
            },
            "Navigation 2",
        ),
        (
            rsx! {
                Navigation2Off {}
            },
            "Navigation 2 Off",
        ),
        (
            rsx! {
                NavigationOff {}
            },
            "Navigation Off",
        ),
        (
            rsx! {
                Network {}
            },
            "Network",
        ),
        (
            rsx! {
                Newspaper {}
            },
            "Newspaper",
        ),
        (
            rsx! {
                Nfc {}
            },
            "Nfc",
        ),
        (
            rsx! {
                NonBinary {}
            },
            "Non Binary",
        ),
        (
            rsx! {
                Notebook {}
            },
            "Notebook",
        ),
        (
            rsx! {
                NotebookPen {}
            },
            "Notebook Pen",
        ),
        (
            rsx! {
                NotebookTabs {}
            },
            "Notebook Tabs",
        ),
        (
            rsx! {
                NotebookText {}
            },
            "Notebook Text",
        ),
        (
            rsx! {
                NotepadText {}
            },
            "Notepad Text",
        ),
        (
            rsx! {
                NotepadTextDashed {}
            },
            "Notepad Text Dashed",
        ),
        (
            rsx! {
                Nut {}
            },
            "Nut",
        ),
        (
            rsx! {
                NutOff {}
            },
            "Nut Off",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsO1() -> Element {
    let icons = [
        (
            rsx! {
                Octagon {}
            },
            "Octagon",
        ),
        (
            rsx! {
                OctagonAlert {}
            },
            "Octagon Alert",
        ),
        (
            rsx! {
                OctagonMinus {}
            },
            "Octagon Minus",
        ),
        (
            rsx! {
                OctagonPause {}
            },
            "Octagon Pause",
        ),
        (
            rsx! {
                OctagonX {}
            },
            "Octagon X",
        ),
        (
            rsx! {
                Omega {}
            },
            "Omega",
        ),
        (
            rsx! {
                Option {}
            },
            "Option",
        ),
        (
            rsx! {
                Orbit {}
            },
            "Orbit",
        ),
        (
            rsx! {
                Origami {}
            },
            "Origami",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsP1() -> Element {
    let icons = [
        (
            rsx! {
                Package {}
            },
            "Package",
        ),
        (
            rsx! {
                Package2 {}
            },
            "Package 2",
        ),
        (
            rsx! {
                PackageCheck {}
            },
            "Package Check",
        ),
        (
            rsx! {
                PackageMinus {}
            },
            "Package Minus",
        ),
        (
            rsx! {
                PackageOpen {}
            },
            "Package Open",
        ),
        (
            rsx! {
                PackagePlus {}
            },
            "Package Plus",
        ),
        (
            rsx! {
                PackageSearch {}
            },
            "Package Search",
        ),
        (
            rsx! {
                PackageX {}
            },
            "Package X",
        ),
        (
            rsx! {
                PaintBucket {}
            },
            "Paint Bucket",
        ),
        (
            rsx! {
                PaintRoller {}
            },
            "Paint Roller",
        ),
        (
            rsx! {
                Paintbrush {}
            },
            "Paintbrush",
        ),
        (
            rsx! {
                PaintbrushVertical {}
            },
            "Paintbrush Vertical",
        ),
        (
            rsx! {
                Palette {}
            },
            "Palette",
        ),
        (
            rsx! {
                Panda {}
            },
            "Panda",
        ),
        (
            rsx! {
                PanelBottom {}
            },
            "Panel Bottom",
        ),
        (
            rsx! {
                PanelBottomClose {}
            },
            "Panel Bottom Close",
        ),
        (
            rsx! {
                PanelBottomDashed {}
            },
            "Panel Bottom Dashed",
        ),
        (
            rsx! {
                PanelBottomOpen {}
            },
            "Panel Bottom Open",
        ),
        (
            rsx! {
                PanelLeft {}
            },
            "Panel Left",
        ),
        (
            rsx! {
                PanelLeftClose {}
            },
            "Panel Left Close",
        ),
        (
            rsx! {
                PanelLeftDashed {}
            },
            "Panel Left Dashed",
        ),
        (
            rsx! {
                PanelLeftOpen {}
            },
            "Panel Left Open",
        ),
        (
            rsx! {
                PanelRight {}
            },
            "Panel Right",
        ),
        (
            rsx! {
                PanelRightClose {}
            },
            "Panel Right Close",
        ),
        (
            rsx! {
                PanelRightDashed {}
            },
            "Panel Right Dashed",
        ),
        (
            rsx! {
                PanelRightOpen {}
            },
            "Panel Right Open",
        ),
        (
            rsx! {
                PanelTop {}
            },
            "Panel Top",
        ),
        (
            rsx! {
                PanelTopClose {}
            },
            "Panel Top Close",
        ),
        (
            rsx! {
                PanelTopDashed {}
            },
            "Panel Top Dashed",
        ),
        (
            rsx! {
                PanelTopOpen {}
            },
            "Panel Top Open",
        ),
        (
            rsx! {
                PanelsLeftBottom {}
            },
            "Panels Left Bottom",
        ),
        (
            rsx! {
                PanelsRightBottom {}
            },
            "Panels Right Bottom",
        ),
        (
            rsx! {
                PanelsTopLeft {}
            },
            "Panels Top Left",
        ),
        (
            rsx! {
                Paperclip {}
            },
            "Paperclip",
        ),
        (
            rsx! {
                Parentheses {}
            },
            "Parentheses",
        ),
        (
            rsx! {
                ParkingMeter {}
            },
            "Parking Meter",
        ),
        (
            rsx! {
                PartyPopper {}
            },
            "Party Popper",
        ),
        (
            rsx! {
                Pause {}
            },
            "Pause",
        ),
        (
            rsx! {
                PawPrint {}
            },
            "Paw Print",
        ),
        (
            rsx! {
                PcCase {}
            },
            "Pc Case",
        ),
        (
            rsx! {
                Pen {}
            },
            "Pen",
        ),
        (
            rsx! {
                PenLine {}
            },
            "Pen Line",
        ),
        (
            rsx! {
                PenOff {}
            },
            "Pen Off",
        ),
        (
            rsx! {
                PenTool {}
            },
            "Pen Tool",
        ),
        (
            rsx! {
                Pencil {}
            },
            "Pencil",
        ),
        (
            rsx! {
                PencilLine {}
            },
            "Pencil Line",
        ),
        (
            rsx! {
                PencilOff {}
            },
            "Pencil Off",
        ),
        (
            rsx! {
                PencilRuler {}
            },
            "Pencil Ruler",
        ),
        (
            rsx! {
                Pentagon {}
            },
            "Pentagon",
        ),
        (
            rsx! {
                Percent {}
            },
            "Percent",
        ),
        (
            rsx! {
                PersonStanding {}
            },
            "Person Standing",
        ),
        (
            rsx! {
                PhilippinePeso {}
            },
            "Philippine Peso",
        ),
        (
            rsx! {
                Phone {}
            },
            "Phone",
        ),
        (
            rsx! {
                PhoneCall {}
            },
            "Phone Call",
        ),
        (
            rsx! {
                PhoneForwarded {}
            },
            "Phone Forwarded",
        ),
        (
            rsx! {
                PhoneIncoming {}
            },
            "Phone Incoming",
        ),
        (
            rsx! {
                PhoneMissed {}
            },
            "Phone Missed",
        ),
        (
            rsx! {
                PhoneOff {}
            },
            "Phone Off",
        ),
        (
            rsx! {
                PhoneOutgoing {}
            },
            "Phone Outgoing",
        ),
        (
            rsx! {
                Pi {}
            },
            "Pi",
        ),
        (
            rsx! {
                Piano {}
            },
            "Piano",
        ),
        (
            rsx! {
                Pickaxe {}
            },
            "Pickaxe",
        ),
        (
            rsx! {
                PictureInPicture {}
            },
            "Picture In Picture",
        ),
        (
            rsx! {
                PictureInPicture2 {}
            },
            "Picture In Picture 2",
        ),
        (
            rsx! {
                PiggyBank {}
            },
            "Piggy Bank",
        ),
        (
            rsx! {
                Pilcrow {}
            },
            "Pilcrow",
        ),
        (
            rsx! {
                PilcrowLeft {}
            },
            "Pilcrow Left",
        ),
        (
            rsx! {
                PilcrowRight {}
            },
            "Pilcrow Right",
        ),
        (
            rsx! {
                Pill {}
            },
            "Pill",
        ),
        (
            rsx! {
                PillBottle {}
            },
            "Pill Bottle",
        ),
        (
            rsx! {
                Pin {}
            },
            "Pin",
        ),
        (
            rsx! {
                PinOff {}
            },
            "Pin Off",
        ),
        (
            rsx! {
                Pipette {}
            },
            "Pipette",
        ),
        (
            rsx! {
                Pizza {}
            },
            "Pizza",
        ),
        (
            rsx! {
                Plane {}
            },
            "Plane",
        ),
        (
            rsx! {
                PlaneLanding {}
            },
            "Plane Landing",
        ),
        (
            rsx! {
                PlaneTakeoff {}
            },
            "Plane Takeoff",
        ),
        (
            rsx! {
                Play {}
            },
            "Play",
        ),
        (
            rsx! {
                Plug {}
            },
            "Plug",
        ),
        (
            rsx! {
                Plug2 {}
            },
            "Plug 2",
        ),
        (
            rsx! {
                PlugZap {}
            },
            "Plug Zap",
        ),
        (
            rsx! {
                Plus {}
            },
            "Plus",
        ),
        (
            rsx! {
                Pocket {}
            },
            "Pocket",
        ),
        (
            rsx! {
                PocketKnife {}
            },
            "Pocket Knife",
        ),
        (
            rsx! {
                Podcast {}
            },
            "Podcast",
        ),
        (
            rsx! {
                Pointer {}
            },
            "Pointer",
        ),
        (
            rsx! {
                PointerOff {}
            },
            "Pointer Off",
        ),
        (
            rsx! {
                Popcorn {}
            },
            "Popcorn",
        ),
        (
            rsx! {
                Popsicle {}
            },
            "Popsicle",
        ),
        (
            rsx! {
                PoundSterling {}
            },
            "Pound Sterling",
        ),
        (
            rsx! {
                Power {}
            },
            "Power",
        ),
        (
            rsx! {
                PowerOff {}
            },
            "Power Off",
        ),
        (
            rsx! {
                Presentation {}
            },
            "Presentation",
        ),
        (
            rsx! {
                Printer {}
            },
            "Printer",
        ),
        (
            rsx! {
                PrinterCheck {}
            },
            "Printer Check",
        ),
        (
            rsx! {
                Projector {}
            },
            "Projector",
        ),
        (
            rsx! {
                Proportions {}
            },
            "Proportions",
        ),
        (
            rsx! {
                Puzzle {}
            },
            "Puzzle",
        ),
        (
            rsx! {
                Pyramid {}
            },
            "Pyramid",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsQ1() -> Element {
    let icons = [
        (
            rsx! {
                QrCode {}
            },
            "Qr Code",
        ),
        (
            rsx! {
                Quote {}
            },
            "Quote",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsR1() -> Element {
    let icons = [
        (
            rsx! {
                Rabbit {}
            },
            "Rabbit",
        ),
        (
            rsx! {
                Radar {}
            },
            "Radar",
        ),
        (
            rsx! {
                Radiation {}
            },
            "Radiation",
        ),
        (
            rsx! {
                Radical {}
            },
            "Radical",
        ),
        (
            rsx! {
                Radio {}
            },
            "Radio",
        ),
        (
            rsx! {
                RadioReceiver {}
            },
            "Radio Receiver",
        ),
        (
            rsx! {
                RadioTower {}
            },
            "Radio Tower",
        ),
        (
            rsx! {
                Radius {}
            },
            "Radius",
        ),
        (
            rsx! {
                RailSymbol {}
            },
            "Rail Symbol",
        ),
        (
            rsx! {
                Rainbow {}
            },
            "Rainbow",
        ),
        (
            rsx! {
                Rat {}
            },
            "Rat",
        ),
        (
            rsx! {
                Ratio {}
            },
            "Ratio",
        ),
        (
            rsx! {
                Receipt {}
            },
            "Receipt",
        ),
        (
            rsx! {
                ReceiptCent {}
            },
            "Receipt Cent",
        ),
        (
            rsx! {
                ReceiptEuro {}
            },
            "Receipt Euro",
        ),
        (
            rsx! {
                ReceiptIndianRupee {}
            },
            "Receipt Indian Rupee",
        ),
        (
            rsx! {
                ReceiptJapaneseYen {}
            },
            "Receipt Japanese Yen",
        ),
        (
            rsx! {
                ReceiptPoundSterling {}
            },
            "Receipt Pound Sterling",
        ),
        (
            rsx! {
                ReceiptRussianRuble {}
            },
            "Receipt Russian Ruble",
        ),
        (
            rsx! {
                ReceiptSwissFranc {}
            },
            "Receipt Swiss Franc",
        ),
        (
            rsx! {
                ReceiptText {}
            },
            "Receipt Text",
        ),
        (
            rsx! {
                RectangleEllipsis {}
            },
            "Rectangle Ellipsis",
        ),
        (
            rsx! {
                RectangleGoggles {}
            },
            "Rectangle Goggles",
        ),
        (
            rsx! {
                RectangleHorizontal {}
            },
            "Rectangle Horizontal",
        ),
        (
            rsx! {
                RectangleVertical {}
            },
            "Rectangle Vertical",
        ),
        (
            rsx! {
                Recycle {}
            },
            "Recycle",
        ),
        (
            rsx! {
                Redo {}
            },
            "Redo",
        ),
        (
            rsx! {
                Redo2 {}
            },
            "Redo 2",
        ),
        (
            rsx! {
                RedoDot {}
            },
            "Redo Dot",
        ),
        (
            rsx! {
                RefreshCcw {}
            },
            "Refresh Ccw",
        ),
        (
            rsx! {
                RefreshCcwDot {}
            },
            "Refresh Ccw Dot",
        ),
        (
            rsx! {
                RefreshCw {}
            },
            "Refresh Cw",
        ),
        (
            rsx! {
                RefreshCwOff {}
            },
            "Refresh Cw Off",
        ),
        (
            rsx! {
                Refrigerator {}
            },
            "Refrigerator",
        ),
        (
            rsx! {
                Regex {}
            },
            "Regex",
        ),
        (
            rsx! {
                RemoveFormatting {}
            },
            "Remove Formatting",
        ),
        (
            rsx! {
                Repeat {}
            },
            "Repeat",
        ),
        (
            rsx! {
                Repeat1 {}
            },
            "Repeat 1",
        ),
        (
            rsx! {
                Repeat2 {}
            },
            "Repeat 2",
        ),
        (
            rsx! {
                Replace {}
            },
            "Replace",
        ),
        (
            rsx! {
                ReplaceAll {}
            },
            "Replace All",
        ),
        (
            rsx! {
                Reply {}
            },
            "Reply",
        ),
        (
            rsx! {
                ReplyAll {}
            },
            "Reply All",
        ),
        (
            rsx! {
                Rewind {}
            },
            "Rewind",
        ),
        (
            rsx! {
                Ribbon {}
            },
            "Ribbon",
        ),
        (
            rsx! {
                Rocket {}
            },
            "Rocket",
        ),
        (
            rsx! {
                RockingChair {}
            },
            "Rocking Chair",
        ),
        (
            rsx! {
                RollerCoaster {}
            },
            "Roller Coaster",
        ),
        (
            rsx! {
                Rotate3D {}
            },
            "Rotate 3 D",
        ),
        (
            rsx! {
                RotateCcw {}
            },
            "Rotate Ccw",
        ),
        (
            rsx! {
                RotateCcwKey {}
            },
            "Rotate Ccw Key",
        ),
        (
            rsx! {
                RotateCcwSquare {}
            },
            "Rotate Ccw Square",
        ),
        (
            rsx! {
                RotateCw {}
            },
            "Rotate Cw",
        ),
        (
            rsx! {
                RotateCwSquare {}
            },
            "Rotate Cw Square",
        ),
        (
            rsx! {
                Route {}
            },
            "Route",
        ),
        (
            rsx! {
                RouteOff {}
            },
            "Route Off",
        ),
        (
            rsx! {
                Router {}
            },
            "Router",
        ),
        (
            rsx! {
                Rows2 {}
            },
            "Rows 2",
        ),
        (
            rsx! {
                Rows3 {}
            },
            "Rows 3",
        ),
        (
            rsx! {
                Rows4 {}
            },
            "Rows 4",
        ),
        (
            rsx! {
                Rss {}
            },
            "Rss",
        ),
        (
            rsx! {
                Ruler {}
            },
            "Ruler",
        ),
        (
            rsx! {
                RulerDimensionLine {}
            },
            "Ruler Dimension Line",
        ),
        (
            rsx! {
                RussianRuble {}
            },
            "Russian Ruble",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsS1() -> Element {
    let icons = [
        (
            rsx! {
                Sailboat {}
            },
            "Sailboat",
        ),
        (
            rsx! {
                Salad {}
            },
            "Salad",
        ),
        (
            rsx! {
                Sandwich {}
            },
            "Sandwich",
        ),
        (
            rsx! {
                Satellite {}
            },
            "Satellite",
        ),
        (
            rsx! {
                SatelliteDish {}
            },
            "Satellite Dish",
        ),
        (
            rsx! {
                SaudiRiyal {}
            },
            "Saudi Riyal",
        ),
        (
            rsx! {
                Save {}
            },
            "Save",
        ),
        (
            rsx! {
                SaveAll {}
            },
            "Save All",
        ),
        (
            rsx! {
                SaveOff {}
            },
            "Save Off",
        ),
        (
            rsx! {
                Scale {}
            },
            "Scale",
        ),
        (
            rsx! {
                Scale3D {}
            },
            "Scale 3 D",
        ),
        (
            rsx! {
                Scaling {}
            },
            "Scaling",
        ),
        (
            rsx! {
                Scan {}
            },
            "Scan",
        ),
        (
            rsx! {
                ScanBarcode {}
            },
            "Scan Barcode",
        ),
        (
            rsx! {
                ScanEye {}
            },
            "Scan Eye",
        ),
        (
            rsx! {
                ScanFace {}
            },
            "Scan Face",
        ),
        (
            rsx! {
                ScanHeart {}
            },
            "Scan Heart",
        ),
        (
            rsx! {
                ScanLine {}
            },
            "Scan Line",
        ),
        (
            rsx! {
                ScanQrCode {}
            },
            "Scan Qr Code",
        ),
        (
            rsx! {
                ScanSearch {}
            },
            "Scan Search",
        ),
        (
            rsx! {
                ScanText {}
            },
            "Scan Text",
        ),
        (
            rsx! {
                School {}
            },
            "School",
        ),
        (
            rsx! {
                Scissors {}
            },
            "Scissors",
        ),
        (
            rsx! {
                ScissorsLineDashed {}
            },
            "Scissors Line Dashed",
        ),
        (
            rsx! {
                ScreenShare {}
            },
            "Screen Share",
        ),
        (
            rsx! {
                ScreenShareOff {}
            },
            "Screen Share Off",
        ),
        (
            rsx! {
                Scroll {}
            },
            "Scroll",
        ),
        (
            rsx! {
                ScrollText {}
            },
            "Scroll Text",
        ),
        (
            rsx! {
                Search {}
            },
            "Search",
        ),
        (
            rsx! {
                SearchCheck {}
            },
            "Search Check",
        ),
        (
            rsx! {
                SearchCode {}
            },
            "Search Code",
        ),
        (
            rsx! {
                SearchSlash {}
            },
            "Search Slash",
        ),
        (
            rsx! {
                SearchX {}
            },
            "Search X",
        ),
        (
            rsx! {
                Section {}
            },
            "Section",
        ),
        (
            rsx! {
                Send {}
            },
            "Send",
        ),
        (
            rsx! {
                SendHorizontal {}
            },
            "Send Horizontal",
        ),
        (
            rsx! {
                SendToBack {}
            },
            "Send To Back",
        ),
        (
            rsx! {
                SeparatorHorizontal {}
            },
            "Separator Horizontal",
        ),
        (
            rsx! {
                SeparatorVertical {}
            },
            "Separator Vertical",
        ),
        (
            rsx! {
                Server {}
            },
            "Server",
        ),
        (
            rsx! {
                ServerCog {}
            },
            "Server Cog",
        ),
        (
            rsx! {
                ServerCrash {}
            },
            "Server Crash",
        ),
        (
            rsx! {
                ServerOff {}
            },
            "Server Off",
        ),
        (
            rsx! {
                Settings {}
            },
            "Settings",
        ),
        (
            rsx! {
                Settings2 {}
            },
            "Settings 2",
        ),
        (
            rsx! {
                Shapes {}
            },
            "Shapes",
        ),
        (
            rsx! {
                Share {}
            },
            "Share",
        ),
        (
            rsx! {
                Share2 {}
            },
            "Share 2",
        ),
        (
            rsx! {
                Sheet {}
            },
            "Sheet",
        ),
        (
            rsx! {
                Shell {}
            },
            "Shell",
        ),
        (
            rsx! {
                Shield {}
            },
            "Shield",
        ),
        (
            rsx! {
                ShieldAlert {}
            },
            "Shield Alert",
        ),
        (
            rsx! {
                ShieldBan {}
            },
            "Shield Ban",
        ),
        (
            rsx! {
                ShieldCheck {}
            },
            "Shield Check",
        ),
        (
            rsx! {
                ShieldEllipsis {}
            },
            "Shield Ellipsis",
        ),
        (
            rsx! {
                ShieldHalf {}
            },
            "Shield Half",
        ),
        (
            rsx! {
                ShieldMinus {}
            },
            "Shield Minus",
        ),
        (
            rsx! {
                ShieldOff {}
            },
            "Shield Off",
        ),
        (
            rsx! {
                ShieldPlus {}
            },
            "Shield Plus",
        ),
        (
            rsx! {
                ShieldQuestion {}
            },
            "Shield Question",
        ),
        (
            rsx! {
                ShieldUser {}
            },
            "Shield User",
        ),
        (
            rsx! {
                ShieldX {}
            },
            "Shield X",
        ),
        (
            rsx! {
                Ship {}
            },
            "Ship",
        ),
        (
            rsx! {
                ShipWheel {}
            },
            "Ship Wheel",
        ),
        (
            rsx! {
                Shirt {}
            },
            "Shirt",
        ),
        (
            rsx! {
                ShoppingBag {}
            },
            "Shopping Bag",
        ),
        (
            rsx! {
                ShoppingBasket {}
            },
            "Shopping Basket",
        ),
        (
            rsx! {
                ShoppingCart {}
            },
            "Shopping Cart",
        ),
        (
            rsx! {
                Shovel {}
            },
            "Shovel",
        ),
        (
            rsx! {
                ShowerHead {}
            },
            "Shower Head",
        ),
        (
            rsx! {
                Shredder {}
            },
            "Shredder",
        ),
        (
            rsx! {
                Shrimp {}
            },
            "Shrimp",
        ),
        (
            rsx! {
                Shrink {}
            },
            "Shrink",
        ),
        (
            rsx! {
                Shrub {}
            },
            "Shrub",
        ),
        (
            rsx! {
                Shuffle {}
            },
            "Shuffle",
        ),
        (
            rsx! {
                Sigma {}
            },
            "Sigma",
        ),
        (
            rsx! {
                Signal {}
            },
            "Signal",
        ),
        (
            rsx! {
                SignalHigh {}
            },
            "Signal High",
        ),
        (
            rsx! {
                SignalLow {}
            },
            "Signal Low",
        ),
        (
            rsx! {
                SignalMedium {}
            },
            "Signal Medium",
        ),
        (
            rsx! {
                SignalZero {}
            },
            "Signal Zero",
        ),
        (
            rsx! {
                Signature {}
            },
            "Signature",
        ),
        (
            rsx! {
                Signpost {}
            },
            "Signpost",
        ),
        (
            rsx! {
                SignpostBig {}
            },
            "Signpost Big",
        ),
        (
            rsx! {
                Siren {}
            },
            "Siren",
        ),
        (
            rsx! {
                SkipBack {}
            },
            "Skip Back",
        ),
        (
            rsx! {
                SkipForward {}
            },
            "Skip Forward",
        ),
        (
            rsx! {
                Skull {}
            },
            "Skull",
        ),
        (
            rsx! {
                Slack {}
            },
            "Slack",
        ),
        (
            rsx! {
                Slash {}
            },
            "Slash",
        ),
        (
            rsx! {
                Slice {}
            },
            "Slice",
        ),
        (
            rsx! {
                SlidersHorizontal {}
            },
            "Sliders Horizontal",
        ),
        (
            rsx! {
                SlidersVertical {}
            },
            "Sliders Vertical",
        ),
        (
            rsx! {
                Smartphone {}
            },
            "Smartphone",
        ),
        (
            rsx! {
                SmartphoneCharging {}
            },
            "Smartphone Charging",
        ),
        (
            rsx! {
                SmartphoneNfc {}
            },
            "Smartphone Nfc",
        ),
        (
            rsx! {
                Smile {}
            },
            "Smile",
        ),
        (
            rsx! {
                SmilePlus {}
            },
            "Smile Plus",
        ),
        (
            rsx! {
                Snail {}
            },
            "Snail",
        ),
        (
            rsx! {
                Snowflake {}
            },
            "Snowflake",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsS2() -> Element {
    let icons = [
        (
            rsx! {
                SoapDispenserDroplet {}
            },
            "Soap Dispenser Droplet",
        ),
        (
            rsx! {
                Sofa {}
            },
            "Sofa",
        ),
        (
            rsx! {
                Soup {}
            },
            "Soup",
        ),
        (
            rsx! {
                Space {}
            },
            "Space",
        ),
        (
            rsx! {
                Spade {}
            },
            "Spade",
        ),
        (
            rsx! {
                Sparkle {}
            },
            "Sparkle",
        ),
        (
            rsx! {
                Sparkles {}
            },
            "Sparkles",
        ),
        (
            rsx! {
                Speaker {}
            },
            "Speaker",
        ),
        (
            rsx! {
                Speech {}
            },
            "Speech",
        ),
        (
            rsx! {
                SpellCheck {}
            },
            "Spell Check",
        ),
        (
            rsx! {
                SpellCheck2 {}
            },
            "Spell Check 2",
        ),
        (
            rsx! {
                Spline {}
            },
            "Spline",
        ),
        (
            rsx! {
                SplinePointer {}
            },
            "Spline Pointer",
        ),
        (
            rsx! {
                Split {}
            },
            "Split",
        ),
        (
            rsx! {
                SprayCan {}
            },
            "Spray Can",
        ),
        (
            rsx! {
                Sprout {}
            },
            "Sprout",
        ),
        (
            rsx! {
                Square {}
            },
            "Square",
        ),
        (
            rsx! {
                SquareActivity {}
            },
            "Square Activity",
        ),
        (
            rsx! {
                SquareArrowDown {}
            },
            "Square Arrow Down",
        ),
        (
            rsx! {
                SquareArrowDownLeft {}
            },
            "Square Arrow Down Left",
        ),
        (
            rsx! {
                SquareArrowDownRight {}
            },
            "Square Arrow Down Right",
        ),
        (
            rsx! {
                SquareArrowLeft {}
            },
            "Square Arrow Left",
        ),
        (
            rsx! {
                SquareArrowOutDownLeft {}
            },
            "Square Arrow Out Down Left",
        ),
        (
            rsx! {
                SquareArrowOutDownRight {}
            },
            "Square Arrow Out Down Right",
        ),
        (
            rsx! {
                SquareArrowOutUpLeft {}
            },
            "Square Arrow Out Up Left",
        ),
        (
            rsx! {
                SquareArrowOutUpRight {}
            },
            "Square Arrow Out Up Right",
        ),
        (
            rsx! {
                SquareArrowRight {}
            },
            "Square Arrow Right",
        ),
        (
            rsx! {
                SquareArrowUp {}
            },
            "Square Arrow Up",
        ),
        (
            rsx! {
                SquareArrowUpLeft {}
            },
            "Square Arrow Up Left",
        ),
        (
            rsx! {
                SquareArrowUpRight {}
            },
            "Square Arrow Up Right",
        ),
        (
            rsx! {
                SquareAsterisk {}
            },
            "Square Asterisk",
        ),
        (
            rsx! {
                SquareBottomDashedScissors {}
            },
            "Square Bottom Dashed Scissors",
        ),
        (
            rsx! {
                SquareChartGantt {}
            },
            "Square Chart Gantt",
        ),
        (
            rsx! {
                SquareCheck {}
            },
            "Square Check",
        ),
        (
            rsx! {
                SquareCheckBig {}
            },
            "Square Check Big",
        ),
        (
            rsx! {
                SquareChevronDown {}
            },
            "Square Chevron Down",
        ),
        (
            rsx! {
                SquareChevronLeft {}
            },
            "Square Chevron Left",
        ),
        (
            rsx! {
                SquareChevronRight {}
            },
            "Square Chevron Right",
        ),
        (
            rsx! {
                SquareChevronUp {}
            },
            "Square Chevron Up",
        ),
        (
            rsx! {
                SquareCode {}
            },
            "Square Code",
        ),
        (
            rsx! {
                SquareDashed {}
            },
            "Square Dashed",
        ),
        (
            rsx! {
                SquareDashedBottom {}
            },
            "Square Dashed Bottom",
        ),
        (
            rsx! {
                SquareDashedBottomCode {}
            },
            "Square Dashed Bottom Code",
        ),
        (
            rsx! {
                SquareDashedKanban {}
            },
            "Square Dashed Kanban",
        ),
        (
            rsx! {
                SquareDashedMousePointer {}
            },
            "Square Dashed Mouse Pointer",
        ),
        (
            rsx! {
                SquareDivide {}
            },
            "Square Divide",
        ),
        (
            rsx! {
                SquareDot {}
            },
            "Square Dot",
        ),
        (
            rsx! {
                SquareEqual {}
            },
            "Square Equal",
        ),
        (
            rsx! {
                SquareFunction {}
            },
            "Square Function",
        ),
        (
            rsx! {
                SquareKanban {}
            },
            "Square Kanban",
        ),
        (
            rsx! {
                SquareLibrary {}
            },
            "Square Library",
        ),
        (
            rsx! {
                SquareM {}
            },
            "Square M",
        ),
        (
            rsx! {
                SquareMenu {}
            },
            "Square Menu",
        ),
        (
            rsx! {
                SquareMinus {}
            },
            "Square Minus",
        ),
        (
            rsx! {
                SquareMousePointer {}
            },
            "Square Mouse Pointer",
        ),
        (
            rsx! {
                SquareParking {}
            },
            "Square Parking",
        ),
        (
            rsx! {
                SquareParkingOff {}
            },
            "Square Parking Off",
        ),
        (
            rsx! {
                SquarePen {}
            },
            "Square Pen",
        ),
        (
            rsx! {
                SquarePercent {}
            },
            "Square Percent",
        ),
        (
            rsx! {
                SquarePi {}
            },
            "Square Pi",
        ),
        (
            rsx! {
                SquarePilcrow {}
            },
            "Square Pilcrow",
        ),
        (
            rsx! {
                SquarePlay {}
            },
            "Square Play",
        ),
        (
            rsx! {
                SquarePlus {}
            },
            "Square Plus",
        ),
        (
            rsx! {
                SquarePower {}
            },
            "Square Power",
        ),
        (
            rsx! {
                SquareRadical {}
            },
            "Square Radical",
        ),
        (
            rsx! {
                SquareRoundCorner {}
            },
            "Square Round Corner",
        ),
        (
            rsx! {
                SquareScissors {}
            },
            "Square Scissors",
        ),
        (
            rsx! {
                SquareSigma {}
            },
            "Square Sigma",
        ),
        (
            rsx! {
                SquareSlash {}
            },
            "Square Slash",
        ),
        (
            rsx! {
                SquareSplitHorizontal {}
            },
            "Square Split Horizontal",
        ),
        (
            rsx! {
                SquareSplitVertical {}
            },
            "Square Split Vertical",
        ),
        (
            rsx! {
                SquareSquare {}
            },
            "Square Square",
        ),
        (
            rsx! {
                SquareStack {}
            },
            "Square Stack",
        ),
        (
            rsx! {
                SquareTerminal {}
            },
            "Square Terminal",
        ),
        (
            rsx! {
                SquareUser {}
            },
            "Square User",
        ),
        (
            rsx! {
                SquareUserRound {}
            },
            "Square User Round",
        ),
        (
            rsx! {
                SquareX {}
            },
            "Square X",
        ),
        (
            rsx! {
                SquaresExclude {}
            },
            "Squares Exclude",
        ),
        (
            rsx! {
                SquaresIntersect {}
            },
            "Squares Intersect",
        ),
        (
            rsx! {
                SquaresSubtract {}
            },
            "Squares Subtract",
        ),
        (
            rsx! {
                SquaresUnite {}
            },
            "Squares Unite",
        ),
        (
            rsx! {
                Squircle {}
            },
            "Squircle",
        ),
        (
            rsx! {
                Squirrel {}
            },
            "Squirrel",
        ),
        (
            rsx! {
                Stamp {}
            },
            "Stamp",
        ),
        (
            rsx! {
                Star {}
            },
            "Star",
        ),
        (
            rsx! {
                StarHalf {}
            },
            "Star Half",
        ),
        (
            rsx! {
                StarOff {}
            },
            "Star Off",
        ),
        (
            rsx! {
                StepBack {}
            },
            "Step Back",
        ),
        (
            rsx! {
                StepForward {}
            },
            "Step Forward",
        ),
        (
            rsx! {
                Stethoscope {}
            },
            "Stethoscope",
        ),
        (
            rsx! {
                Sticker {}
            },
            "Sticker",
        ),
        (
            rsx! {
                StickyNote {}
            },
            "Sticky Note",
        ),
        (
            rsx! {
                Store {}
            },
            "Store",
        ),
        (
            rsx! {
                StretchHorizontal {}
            },
            "Stretch Horizontal",
        ),
        (
            rsx! {
                StretchVertical {}
            },
            "Stretch Vertical",
        ),
        (
            rsx! {
                Strikethrough {}
            },
            "Strikethrough",
        ),
        (
            rsx! {
                Subscript {}
            },
            "Subscript",
        ),
        (
            rsx! {
                Sun {}
            },
            "Sun",
        ),
        (
            rsx! {
                SunDim {}
            },
            "Sun Dim",
        ),
        (
            rsx! {
                SunMedium {}
            },
            "Sun Medium",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsS3() -> Element {
    let icons = [
        (
            rsx! {
                SunMoon {}
            },
            "Sun Moon",
        ),
        (
            rsx! {
                SunSnow {}
            },
            "Sun Snow",
        ),
        (
            rsx! {
                Sunrise {}
            },
            "Sunrise",
        ),
        (
            rsx! {
                Sunset {}
            },
            "Sunset",
        ),
        (
            rsx! {
                Superscript {}
            },
            "Superscript",
        ),
        (
            rsx! {
                SwatchBook {}
            },
            "Swatch Book",
        ),
        (
            rsx! {
                SwissFranc {}
            },
            "Swiss Franc",
        ),
        (
            rsx! {
                SwitchCamera {}
            },
            "Switch Camera",
        ),
        (
            rsx! {
                Sword {}
            },
            "Sword",
        ),
        (
            rsx! {
                Swords {}
            },
            "Swords",
        ),
        (
            rsx! {
                Syringe {}
            },
            "Syringe",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsT1() -> Element {
    let icons = [
        (
            rsx! {
                Table {}
            },
            "Table",
        ),
        (
            rsx! {
                Table2 {}
            },
            "Table 2",
        ),
        (
            rsx! {
                TableCellsMerge {}
            },
            "Table Cells Merge",
        ),
        (
            rsx! {
                TableCellsSplit {}
            },
            "Table Cells Split",
        ),
        (
            rsx! {
                TableColumnsSplit {}
            },
            "Table Columns Split",
        ),
        (
            rsx! {
                TableOfContents {}
            },
            "Table Of Contents",
        ),
        (
            rsx! {
                TableProperties {}
            },
            "Table Properties",
        ),
        (
            rsx! {
                TableRowsSplit {}
            },
            "Table Rows Split",
        ),
        (
            rsx! {
                Tablet {}
            },
            "Tablet",
        ),
        (
            rsx! {
                TabletSmartphone {}
            },
            "Tablet Smartphone",
        ),
        (
            rsx! {
                Tablets {}
            },
            "Tablets",
        ),
        (
            rsx! {
                Tag {}
            },
            "Tag",
        ),
        (
            rsx! {
                Tags {}
            },
            "Tags",
        ),
        (
            rsx! {
                Tally1 {}
            },
            "Tally 1",
        ),
        (
            rsx! {
                Tally2 {}
            },
            "Tally 2",
        ),
        (
            rsx! {
                Tally3 {}
            },
            "Tally 3",
        ),
        (
            rsx! {
                Tally4 {}
            },
            "Tally 4",
        ),
        (
            rsx! {
                Tally5 {}
            },
            "Tally 5",
        ),
        (
            rsx! {
                Tangent {}
            },
            "Tangent",
        ),
        (
            rsx! {
                Target {}
            },
            "Target",
        ),
        (
            rsx! {
                Telescope {}
            },
            "Telescope",
        ),
        (
            rsx! {
                Tent {}
            },
            "Tent",
        ),
        (
            rsx! {
                TentTree {}
            },
            "Tent Tree",
        ),
        (
            rsx! {
                Terminal {}
            },
            "Terminal",
        ),
        (
            rsx! {
                TestTube {}
            },
            "Test Tube",
        ),
        (
            rsx! {
                TestTubeDiagonal {}
            },
            "Test Tube Diagonal",
        ),
        (
            rsx! {
                TestTubes {}
            },
            "Test Tubes",
        ),
        (
            rsx! {
                Text {}
            },
            "Text",
        ),
        (
            rsx! {
                TextCursor {}
            },
            "Text Cursor",
        ),
        (
            rsx! {
                TextCursorInput {}
            },
            "Text Cursor Input",
        ),
        (
            rsx! {
                TextQuote {}
            },
            "Text Quote",
        ),
        (
            rsx! {
                TextSearch {}
            },
            "Text Search",
        ),
        (
            rsx! {
                TextSelect {}
            },
            "Text Select",
        ),
        (
            rsx! {
                Theater {}
            },
            "Theater",
        ),
        (
            rsx! {
                Thermometer {}
            },
            "Thermometer",
        ),
        (
            rsx! {
                ThermometerSnowflake {}
            },
            "Thermometer Snowflake",
        ),
        (
            rsx! {
                ThermometerSun {}
            },
            "Thermometer Sun",
        ),
        (
            rsx! {
                ThumbsDown {}
            },
            "Thumbs Down",
        ),
        (
            rsx! {
                ThumbsUp {}
            },
            "Thumbs Up",
        ),
        (
            rsx! {
                Ticket {}
            },
            "Ticket",
        ),
        (
            rsx! {
                TicketCheck {}
            },
            "Ticket Check",
        ),
        (
            rsx! {
                TicketMinus {}
            },
            "Ticket Minus",
        ),
        (
            rsx! {
                TicketPercent {}
            },
            "Ticket Percent",
        ),
        (
            rsx! {
                TicketPlus {}
            },
            "Ticket Plus",
        ),
        (
            rsx! {
                TicketSlash {}
            },
            "Ticket Slash",
        ),
        (
            rsx! {
                TicketX {}
            },
            "Ticket X",
        ),
        (
            rsx! {
                Tickets {}
            },
            "Tickets",
        ),
        (
            rsx! {
                TicketsPlane {}
            },
            "Tickets Plane",
        ),
        (
            rsx! {
                Timer {}
            },
            "Timer",
        ),
        (
            rsx! {
                TimerOff {}
            },
            "Timer Off",
        ),
        (
            rsx! {
                TimerReset {}
            },
            "Timer Reset",
        ),
        (
            rsx! {
                ToggleLeft {}
            },
            "Toggle Left",
        ),
        (
            rsx! {
                ToggleRight {}
            },
            "Toggle Right",
        ),
        (
            rsx! {
                Toilet {}
            },
            "Toilet",
        ),
        (
            rsx! {
                Tornado {}
            },
            "Tornado",
        ),
        (
            rsx! {
                Torus {}
            },
            "Torus",
        ),
        (
            rsx! {
                Touchpad {}
            },
            "Touchpad",
        ),
        (
            rsx! {
                TouchpadOff {}
            },
            "Touchpad Off",
        ),
        (
            rsx! {
                TowerControl {}
            },
            "Tower Control",
        ),
        (
            rsx! {
                ToyBrick {}
            },
            "Toy Brick",
        ),
        (
            rsx! {
                Tractor {}
            },
            "Tractor",
        ),
        (
            rsx! {
                TrafficCone {}
            },
            "Traffic Cone",
        ),
        (
            rsx! {
                TrainFront {}
            },
            "Train Front",
        ),
        (
            rsx! {
                TrainFrontTunnel {}
            },
            "Train Front Tunnel",
        ),
        (
            rsx! {
                TrainTrack {}
            },
            "Train Track",
        ),
        (
            rsx! {
                TramFront {}
            },
            "Tram Front",
        ),
        (
            rsx! {
                Transgender {}
            },
            "Transgender",
        ),
        (
            rsx! {
                Trash {}
            },
            "Trash",
        ),
        (
            rsx! {
                Trash2 {}
            },
            "Trash 2",
        ),
        (
            rsx! {
                TreeDeciduous {}
            },
            "Tree Deciduous",
        ),
        (
            rsx! {
                TreePalm {}
            },
            "Tree Palm",
        ),
        (
            rsx! {
                TreePine {}
            },
            "Tree Pine",
        ),
        (
            rsx! {
                Trees {}
            },
            "Trees",
        ),
        (
            rsx! {
                Trello {}
            },
            "Trello",
        ),
        (
            rsx! {
                TrendingDown {}
            },
            "Trending Down",
        ),
        (
            rsx! {
                TrendingUp {}
            },
            "Trending Up",
        ),
        (
            rsx! {
                TrendingUpDown {}
            },
            "Trending Up Down",
        ),
        (
            rsx! {
                Triangle {}
            },
            "Triangle",
        ),
        (
            rsx! {
                TriangleAlert {}
            },
            "Triangle Alert",
        ),
        (
            rsx! {
                TriangleDashed {}
            },
            "Triangle Dashed",
        ),
        (
            rsx! {
                TriangleRight {}
            },
            "Triangle Right",
        ),
        (
            rsx! {
                Trophy {}
            },
            "Trophy",
        ),
        (
            rsx! {
                Truck {}
            },
            "Truck",
        ),
        (
            rsx! {
                TruckElectric {}
            },
            "Truck Electric",
        ),
        (
            rsx! {
                Turtle {}
            },
            "Turtle",
        ),
        (
            rsx! {
                Tv {}
            },
            "Tv",
        ),
        (
            rsx! {
                TvMinimal {}
            },
            "Tv Minimal",
        ),
        (
            rsx! {
                TvMinimalPlay {}
            },
            "Tv Minimal Play",
        ),
        (
            rsx! {
                Twitch {}
            },
            "Twitch",
        ),
        (
            rsx! {
                Twitter {}
            },
            "Twitter",
        ),
        (
            rsx! {
                Type {}
            },
            "Type",
        ),
        (
            rsx! {
                TypeOutline {}
            },
            "Type Outline",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsU1() -> Element {
    let icons = [
        (
            rsx! {
                Umbrella {}
            },
            "Umbrella",
        ),
        (
            rsx! {
                UmbrellaOff {}
            },
            "Umbrella Off",
        ),
        (
            rsx! {
                Underline {}
            },
            "Underline",
        ),
        (
            rsx! {
                Undo {}
            },
            "Undo",
        ),
        (
            rsx! {
                Undo2 {}
            },
            "Undo 2",
        ),
        (
            rsx! {
                UndoDot {}
            },
            "Undo Dot",
        ),
        (
            rsx! {
                UnfoldHorizontal {}
            },
            "Unfold Horizontal",
        ),
        (
            rsx! {
                UnfoldVertical {}
            },
            "Unfold Vertical",
        ),
        (
            rsx! {
                Ungroup {}
            },
            "Ungroup",
        ),
        (
            rsx! {
                University {}
            },
            "University",
        ),
        (
            rsx! {
                Unlink {}
            },
            "Unlink",
        ),
        (
            rsx! {
                Unlink2 {}
            },
            "Unlink 2",
        ),
        (
            rsx! {
                Unplug {}
            },
            "Unplug",
        ),
        (
            rsx! {
                Upload {}
            },
            "Upload",
        ),
        (
            rsx! {
                Usb {}
            },
            "Usb",
        ),
        (
            rsx! {
                User {}
            },
            "User",
        ),
        (
            rsx! {
                UserCheck {}
            },
            "User Check",
        ),
        (
            rsx! {
                UserCog {}
            },
            "User Cog",
        ),
        (
            rsx! {
                UserLock {}
            },
            "User Lock",
        ),
        (
            rsx! {
                UserMinus {}
            },
            "User Minus",
        ),
        (
            rsx! {
                UserPen {}
            },
            "User Pen",
        ),
        (
            rsx! {
                UserPlus {}
            },
            "User Plus",
        ),
        (
            rsx! {
                UserRound {}
            },
            "User Round",
        ),
        (
            rsx! {
                UserRoundCheck {}
            },
            "User Round Check",
        ),
        (
            rsx! {
                UserRoundCog {}
            },
            "User Round Cog",
        ),
        (
            rsx! {
                UserRoundMinus {}
            },
            "User Round Minus",
        ),
        (
            rsx! {
                UserRoundPen {}
            },
            "User Round Pen",
        ),
        (
            rsx! {
                UserRoundPlus {}
            },
            "User Round Plus",
        ),
        (
            rsx! {
                UserRoundSearch {}
            },
            "User Round Search",
        ),
        (
            rsx! {
                UserRoundX {}
            },
            "User Round X",
        ),
        (
            rsx! {
                UserSearch {}
            },
            "User Search",
        ),
        (
            rsx! {
                UserX {}
            },
            "User X",
        ),
        (
            rsx! {
                Users {}
            },
            "Users",
        ),
        (
            rsx! {
                UsersRound {}
            },
            "Users Round",
        ),
        (
            rsx! {
                Utensils {}
            },
            "Utensils",
        ),
        (
            rsx! {
                UtensilsCrossed {}
            },
            "Utensils Crossed",
        ),
        (
            rsx! {
                UtilityPole {}
            },
            "Utility Pole",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsV1() -> Element {
    let icons = [
        (
            rsx! {
                Variable {}
            },
            "Variable",
        ),
        (
            rsx! {
                Vault {}
            },
            "Vault",
        ),
        (
            rsx! {
                Vegan {}
            },
            "Vegan",
        ),
        (
            rsx! {
                VenetianMask {}
            },
            "Venetian Mask",
        ),
        (
            rsx! {
                Venus {}
            },
            "Venus",
        ),
        (
            rsx! {
                VenusAndMars {}
            },
            "Venus And Mars",
        ),
        (
            rsx! {
                Vibrate {}
            },
            "Vibrate",
        ),
        (
            rsx! {
                VibrateOff {}
            },
            "Vibrate Off",
        ),
        (
            rsx! {
                Video {}
            },
            "Video",
        ),
        (
            rsx! {
                VideoOff {}
            },
            "Video Off",
        ),
        (
            rsx! {
                Videotape {}
            },
            "Videotape",
        ),
        (
            rsx! {
                View {}
            },
            "View",
        ),
        (
            rsx! {
                Voicemail {}
            },
            "Voicemail",
        ),
        (
            rsx! {
                Volleyball {}
            },
            "Volleyball",
        ),
        (
            rsx! {
                Volume {}
            },
            "Volume",
        ),
        (
            rsx! {
                Volume1 {}
            },
            "Volume 1",
        ),
        (
            rsx! {
                Volume2 {}
            },
            "Volume 2",
        ),
        (
            rsx! {
                VolumeOff {}
            },
            "Volume Off",
        ),
        (
            rsx! {
                VolumeX {}
            },
            "Volume X",
        ),
        (
            rsx! {
                Vote {}
            },
            "Vote",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsW1() -> Element {
    let icons = [
        (
            rsx! {
                Wallet {}
            },
            "Wallet",
        ),
        (
            rsx! {
                WalletCards {}
            },
            "Wallet Cards",
        ),
        (
            rsx! {
                WalletMinimal {}
            },
            "Wallet Minimal",
        ),
        (
            rsx! {
                Wallpaper {}
            },
            "Wallpaper",
        ),
        (
            rsx! {
                Wand {}
            },
            "Wand",
        ),
        (
            rsx! {
                WandSparkles {}
            },
            "Wand Sparkles",
        ),
        (
            rsx! {
                Warehouse {}
            },
            "Warehouse",
        ),
        (
            rsx! {
                WashingMachine {}
            },
            "Washing Machine",
        ),
        (
            rsx! {
                Watch {}
            },
            "Watch",
        ),
        (
            rsx! {
                Waves {}
            },
            "Waves",
        ),
        (
            rsx! {
                WavesLadder {}
            },
            "Waves Ladder",
        ),
        (
            rsx! {
                Waypoints {}
            },
            "Waypoints",
        ),
        (
            rsx! {
                Webcam {}
            },
            "Webcam",
        ),
        (
            rsx! {
                Webhook {}
            },
            "Webhook",
        ),
        (
            rsx! {
                WebhookOff {}
            },
            "Webhook Off",
        ),
        (
            rsx! {
                Weight {}
            },
            "Weight",
        ),
        (
            rsx! {
                Wheat {}
            },
            "Wheat",
        ),
        (
            rsx! {
                WheatOff {}
            },
            "Wheat Off",
        ),
        (
            rsx! {
                WholeWord {}
            },
            "Whole Word",
        ),
        (
            rsx! {
                Wifi {}
            },
            "Wifi",
        ),
        (
            rsx! {
                WifiHigh {}
            },
            "Wifi High",
        ),
        (
            rsx! {
                WifiLow {}
            },
            "Wifi Low",
        ),
        (
            rsx! {
                WifiOff {}
            },
            "Wifi Off",
        ),
        (
            rsx! {
                WifiPen {}
            },
            "Wifi Pen",
        ),
        (
            rsx! {
                WifiZero {}
            },
            "Wifi Zero",
        ),
        (
            rsx! {
                Wind {}
            },
            "Wind",
        ),
        (
            rsx! {
                WindArrowDown {}
            },
            "Wind Arrow Down",
        ),
        (
            rsx! {
                Wine {}
            },
            "Wine",
        ),
        (
            rsx! {
                WineOff {}
            },
            "Wine Off",
        ),
        (
            rsx! {
                Workflow {}
            },
            "Workflow",
        ),
        (
            rsx! {
                Worm {}
            },
            "Worm",
        ),
        (
            rsx! {
                WrapText {}
            },
            "Wrap Text",
        ),
        (
            rsx! {
                Wrench {}
            },
            "Wrench",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsX1() -> Element {
    let icons = [(
        rsx! {
            X {}
        },
        "X",
    )];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsY1() -> Element {
    let icons = [(
        rsx! {
            Youtube {}
        },
        "Youtube",
    )];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
#[component]
pub fn IconsZ1() -> Element {
    let icons = [
        (
            rsx! {
                Zap {}
            },
            "Zap",
        ),
        (
            rsx! {
                ZapOff {}
            },
            "Zap Off",
        ),
        (
            rsx! {
                ZoomIn {}
            },
            "Zoom In",
        ),
        (
            rsx! {
                ZoomOut {}
            },
            "Zoom Out",
        ),
    ];
    rsx! {
        for (icon , name) in icons {
            div {
                key: "{name}",
                class: "flex flex-wrap items-center gap-4 text-sm",
                {icon}
                span { {name} }
            }
        }
    }
}
