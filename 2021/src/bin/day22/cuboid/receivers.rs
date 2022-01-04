use super::Cuboid;

impl Cuboid {
    /// Inclusive
    fn above(&self, z: i32) -> Cuboid {
        let zs = z..self.zs.end;
        Cuboid { zs, ..self.clone() }
    }

    /// Exclusive
    fn below(&self, z: i32) -> Cuboid {
        let zs = self.zs.start..z;
        Cuboid { zs, ..self.clone() }
    }

    /// Exclusive
    fn left_of(&self, x: i32) -> Cuboid {
        let xs = self.xs.start..x;
        Cuboid { xs, ..self.clone() }
    }
}

impl Cuboid {
    /// Clips the outermost corner off self.  Returns the remainder as three
    /// cuboids: the part directly below the new cuboid (nearer the XY plane),
    /// the part behind that and the new cuboid (nearer the XZ plane), and the
    /// part to the left of those (nearer the YZ plane).
    fn clip111(self, new: &Cuboid) -> Vec<Cuboid> {
        let below_new = Cuboid {
            xs: new.xs.start..self.xs.end,
            ys: new.ys.start..self.ys.end,
            zs: self.zs.start..new.zs.start,
        };
        let before = Cuboid {
            xs: new.xs.start..self.xs.end,
            ys: self.ys.start..new.ys.start,
            zs: self.zs.clone(),
        };
        vec![below_new, before, self.left_of(new.xs.start)]
    }

    /// Clips a chunk out of self in the right, far edge.  Returns the
    /// remainder as four cuboids:  the entire top of self (above new.zs.end),
    /// and the result of clipping the outermost corner of what remains (self
    /// below new.zs.end).
    fn clip113(self, new: &Cuboid) -> Vec<Cuboid> {
        let mut result = self.below(new.zs.end).clip111(new);
        result.push(self.above(new.zs.end));
        result
    }

    /// Clips the right, far quadrant off self.  Returns the result as two
    /// cuboids: The part directly before the new cuboid (closer to the XZ
    /// plane), and the remainder of self (closer to the YZ plane).
    fn clip114(self, new: &Cuboid) -> Vec<Cuboid> {
        debug_assert_eq!(self.zs.start, new.zs.start);
        let before = Cuboid {
            xs: new.xs.start..self.xs.end,
            ys: self.ys.start..new.ys.start,
            zs: self.zs.clone(),
        };
        vec![before, self.left_of(new.xs.start)]
    }

    /// Clips the right, far, bottom corner off self.
    fn clip116(self, new: &Cuboid) -> Vec<Cuboid> {
        let mut result = self.below(new.zs.end).clip114(new);
        result.push(self.above(new.zs.end));
        result
    }

    fn empty(self, _new: &Cuboid) -> Vec<Cuboid> {
        vec![]
    }

    fn identity(self, _new: &Cuboid) -> Vec<Cuboid> {
        vec![self]
    }

    fn todo(self, _new: &Cuboid) -> Vec<Cuboid> {
        todo!()
    }
}

// 0 means no overlap; so, any index containing a 0 digit maps to the identity
// function, since the new cuboid misses the old one enitrely.
//
// 4, 5, 7, and 8, mean the old cube does not extend beyond the new one; so,
// any index containing only those digits maps to the empty function, since the
// new cuboid obliterates the old one entirely.
pub const RECEIVERS: &[fn(Cuboid, &Cuboid) -> Vec<Cuboid>] = &[
    Cuboid::identity, // 000
    Cuboid::identity, // 001
    Cuboid::identity, // 002
    Cuboid::identity, // 003
    Cuboid::identity, // 004
    Cuboid::identity, // 005
    Cuboid::identity, // 006
    Cuboid::identity, // 007
    Cuboid::identity, // 008
    Cuboid::identity, // 009
    Cuboid::identity, // 010
    Cuboid::identity, // 011
    Cuboid::identity, // 012
    Cuboid::identity, // 013
    Cuboid::identity, // 014
    Cuboid::identity, // 015
    Cuboid::identity, // 016
    Cuboid::identity, // 017
    Cuboid::identity, // 018
    Cuboid::identity, // 019
    Cuboid::identity, // 020
    Cuboid::identity, // 021
    Cuboid::identity, // 022
    Cuboid::identity, // 023
    Cuboid::identity, // 024
    Cuboid::identity, // 025
    Cuboid::identity, // 026
    Cuboid::identity, // 027
    Cuboid::identity, // 028
    Cuboid::identity, // 029
    Cuboid::identity, // 030
    Cuboid::identity, // 031
    Cuboid::identity, // 032
    Cuboid::identity, // 033
    Cuboid::identity, // 034
    Cuboid::identity, // 035
    Cuboid::identity, // 036
    Cuboid::identity, // 037
    Cuboid::identity, // 038
    Cuboid::identity, // 039
    Cuboid::identity, // 040
    Cuboid::identity, // 041
    Cuboid::identity, // 042
    Cuboid::identity, // 043
    Cuboid::identity, // 044
    Cuboid::identity, // 045
    Cuboid::identity, // 046
    Cuboid::identity, // 047
    Cuboid::identity, // 048
    Cuboid::identity, // 049
    Cuboid::identity, // 050
    Cuboid::identity, // 051
    Cuboid::identity, // 052
    Cuboid::identity, // 053
    Cuboid::identity, // 054
    Cuboid::identity, // 055
    Cuboid::identity, // 056
    Cuboid::identity, // 057
    Cuboid::identity, // 058
    Cuboid::identity, // 059
    Cuboid::identity, // 060
    Cuboid::identity, // 061
    Cuboid::identity, // 062
    Cuboid::identity, // 063
    Cuboid::identity, // 064
    Cuboid::identity, // 065
    Cuboid::identity, // 066
    Cuboid::identity, // 067
    Cuboid::identity, // 068
    Cuboid::identity, // 069
    Cuboid::identity, // 070
    Cuboid::identity, // 071
    Cuboid::identity, // 072
    Cuboid::identity, // 073
    Cuboid::identity, // 074
    Cuboid::identity, // 075
    Cuboid::identity, // 076
    Cuboid::identity, // 077
    Cuboid::identity, // 078
    Cuboid::identity, // 079
    Cuboid::identity, // 080
    Cuboid::identity, // 081
    Cuboid::identity, // 082
    Cuboid::identity, // 083
    Cuboid::identity, // 084
    Cuboid::identity, // 085
    Cuboid::identity, // 086
    Cuboid::identity, // 087
    Cuboid::identity, // 088
    Cuboid::identity, // 089
    Cuboid::identity, // 090
    Cuboid::identity, // 091
    Cuboid::identity, // 092
    Cuboid::identity, // 093
    Cuboid::identity, // 094
    Cuboid::identity, // 095
    Cuboid::identity, // 096
    Cuboid::identity, // 097
    Cuboid::identity, // 098
    Cuboid::identity, // 099
    Cuboid::identity, // 100
    Cuboid::identity, // 101
    Cuboid::identity, // 102
    Cuboid::identity, // 103
    Cuboid::identity, // 104
    Cuboid::identity, // 105
    Cuboid::identity, // 106
    Cuboid::identity, // 107
    Cuboid::identity, // 108
    Cuboid::identity, // 109
    Cuboid::identity, // 110
    Cuboid::clip111,  // 111
    Cuboid::clip111,  // 112
    Cuboid::clip113,  // 113
    Cuboid::clip114,  // 114
    Cuboid::clip114,  // 115
    Cuboid::clip116,  // 116
    //
    Cuboid::clip111, // 121
    Cuboid::clip111, // 122
    //
    Cuboid::clip111, // 211
    Cuboid::clip111, // 212
    //
    Cuboid::clip111, // 221
    Cuboid::clip111, // 222
    //
    Cuboid::empty, // 444
    Cuboid::empty, // 445
    //
    Cuboid::empty, // 447
    Cuboid::empty, // 448
    //
    Cuboid::empty, // 454
    Cuboid::empty, // 455
    //
    Cuboid::empty, // 457
    Cuboid::empty, // 458
    //
    Cuboid::empty, // 474
    Cuboid::empty, // 475
    //
    Cuboid::empty, // 477
    Cuboid::empty, // 478
    //
    Cuboid::empty, // 484
    Cuboid::empty, // 485
    //
    Cuboid::empty, // 487
    Cuboid::empty, // 488
    //
    Cuboid::empty, // 544
    Cuboid::empty, // 545
    //
    Cuboid::empty, // 547
    Cuboid::empty, // 548
    //
    Cuboid::empty, // 554
    Cuboid::empty, // 555
    //
    Cuboid::empty, // 557
    Cuboid::empty, // 558
    //
    Cuboid::empty, // 574
    Cuboid::empty, // 575
    //
    Cuboid::empty, // 577
    Cuboid::empty, // 578
    //
    Cuboid::empty, // 584
    Cuboid::empty, // 585
    //
    Cuboid::empty, // 587
    Cuboid::empty, // 588
    //
    Cuboid::empty, // 744
    Cuboid::empty, // 745
    //
    Cuboid::empty, // 747
    Cuboid::empty, // 748
    //
    Cuboid::empty, // 754
    Cuboid::empty, // 755
    //
    Cuboid::empty, // 757
    Cuboid::empty, // 758
    //
    Cuboid::empty, // 774
    Cuboid::empty, // 775
    //
    Cuboid::empty, // 777
    Cuboid::empty, // 778
    //
    Cuboid::empty, // 784
    Cuboid::empty, // 785
    //
    Cuboid::empty, // 787
    Cuboid::empty, // 788
    //
    Cuboid::empty, // 844
    Cuboid::empty, // 845
    //
    Cuboid::empty, // 847
    Cuboid::empty, // 848
    //
    Cuboid::empty, // 854
    Cuboid::empty, // 855
    //
    Cuboid::empty, // 857
    Cuboid::empty, // 858
    //
    Cuboid::empty, // 874
    Cuboid::empty, // 875
    //
    Cuboid::empty, // 877
    Cuboid::empty, // 878
    //
    Cuboid::empty, // 884
    Cuboid::empty, // 885
    //
    Cuboid::empty, // 887
    Cuboid::empty, // 888
];
