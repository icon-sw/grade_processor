use num_traits::{Float, Zero, One};

const SEMI_AXIS_MAJOR: f64 = 6378137.0;
const SEMI_AXIS_MINOR: f64 = 6356752.314245179;
const FLATTENING: f64 = 1.0/298.257223563;
const ECCENTRICITY_SQUARE: f64 = FLATTENING * (2.0 - FLATTENING);

#[derive(Clone, Debug)]
pub struct RAEPoint<T: Float> {
    pub range: T,
    pub azimuth: T,
    pub elevation: T,
}

#[derive(Clone, Debug)]
pub struct XYZPoint<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone, Debug)]
pub struct ECEFPoint<T: Float> {
    pub x_ecef: T,
    pub y_ecef: T,
    pub z_ecef: T,
}

#[derive(Clone, Debug)]
pub struct LLAPoint<T: Float> {
    pub latitude: T,
    pub longitude: T,
    pub altitude: T,
}

impl<T: Float + Zero> RAEPoint<T> {
    pub fn new(range: T, azimuth: T, elevation: T) -> Self {
        RAEPoint { range, azimuth, elevation }
    }
    
    pub fn distance_from(&self, other: RAEPoint<T>) -> T {
        let p1 = XYZPoint::<T>::from(self.clone());
        let p2 = XYZPoint::<T>::from(other);

        p1.distance_from(p2)
    } 

    pub fn from_ecef(ecef: ECEFPoint<T>, origin: ECEFPoint<T>) -> Self {
        XYZPoint::from_ecef(ecef, origin).into()
    }

    pub fn to_ecef(&self, reference: ECEFPoint<T>) -> ECEFPoint<T> {
        let xyz: XYZPoint<T> = self.clone().into();
        ECEFPoint::from_xyz(xyz, reference)
    }

    pub fn from_lla(lla: LLAPoint<T>, reference: LLAPoint<T>) -> Self
    where T: std::convert::From<f64> {
        let ecef: ECEFPoint<T> = lla.into();
        RAEPoint::from_ecef(ecef, reference.into())
    }

    pub fn to_lla(&self, reference: LLAPoint<T>) -> LLAPoint<T> 
    where T: std::convert::From<f64> {
        self.to_ecef(reference.into()).into()
    }
}

impl<T: Float> XYZPoint<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        XYZPoint { x, y, z }
    }

    pub fn distance_from(&self, other: XYZPoint<T>) -> T {
        ((self.x - other.x).powi(2) + 
         (self.y - other.y).powi(2) + 
         (self.z - other.z).powi(2)).sqrt()
    }

    pub fn from_ecef(ecef: ECEFPoint<T>, reference: ECEFPoint<T>) -> Self {
        let x = ecef.x_ecef - reference.x_ecef;
        let y = ecef.y_ecef - reference.y_ecef;
        let z = ecef.z_ecef - reference.z_ecef;
        XYZPoint { x, y, z }
    }

    pub fn to_ecef(&self, reference: ECEFPoint<T>) -> ECEFPoint<T> {
        ECEFPoint::new(self.x + reference.x_ecef, 
                       self.y + reference.y_ecef, 
                       self.z + reference.z_ecef)
    }

    pub fn from_lla(lla: LLAPoint<T>, reference: LLAPoint<T>) -> Self
    where T: std::convert::From<f64> {
        let ecef: ECEFPoint<T> = lla.into();
        XYZPoint::from_ecef(ecef, reference.into())
    }

    pub fn to_lla(&self, reference: LLAPoint<T>) -> LLAPoint<T> 
    where T: std::convert::From<f64> {
        self.to_ecef(reference.into()).into()
    }
}

impl<T: Float> ECEFPoint<T> {
    pub fn new(x_ecef: T, y_ecef: T, z_ecef: T) -> Self {
        ECEFPoint { x_ecef, y_ecef, z_ecef }
    }
    pub fn distance_from(&self, other: ECEFPoint<T>) -> T {
        ((self.x_ecef - other.x_ecef).powi(2) + 
         (self.y_ecef - other.y_ecef).powi(2) + 
         (self.z_ecef - other.z_ecef).powi(2)).sqrt()
    }

    pub fn from_rae(rae: RAEPoint<T>, reference: ECEFPoint<T>) -> Self {
        let xyz: XYZPoint<T> = rae.into();
        ECEFPoint::from_xyz(xyz, reference)
    }

    pub fn to_rae(&self, reference: ECEFPoint<T>) -> RAEPoint<T> {
        let xyz: XYZPoint<T> = self.to_xyz(reference);
        RAEPoint::from(xyz)
    }

    pub fn from_xyz(xyz: XYZPoint<T>, reference: ECEFPoint<T>) -> Self {
        let x_ecef = xyz.x + reference.x_ecef;
        let y_ecef = xyz.y + reference.y_ecef;
        let z_ecef = xyz.z + reference.z_ecef;
        ECEFPoint { x_ecef, y_ecef, z_ecef }
    }

    pub fn to_xyz(&self, reference: ECEFPoint<T>) -> XYZPoint<T> {
        XYZPoint::new(self.x_ecef - reference.x_ecef, 
                      self.y_ecef - reference.y_ecef, 
                      self.z_ecef - reference.z_ecef)
    }
}

impl<T: Float> LLAPoint<T> {
    pub fn new(latitude: T, longitude: T, altitude: T) -> Self {
        LLAPoint { latitude, longitude, altitude }
    }

    pub fn distance_from(&self, other: LLAPoint<T>) -> T 
    where T: std::convert::From<f64> {
        let ecef1: ECEFPoint<T> = self.clone().into();
        let ecef2: ECEFPoint<T> = other.into();
        ecef1.distance_from(ecef2)
    }

    pub fn haversine_distance(&self, other: LLAPoint<T>) -> T 
    where T: std::convert::From<f64> {

        let dlat_half = (self.latitude - other.latitude).to_radians()/2.0_f64.into();
        let dlon_half = (self.longitude - other.longitude).to_radians()/2.0_f64.into();

        let a: T = dlat_half.sin().powi(2) 
                + ( self.latitude.to_degrees().cos() 
                    * other.latitude.to_degrees().cos() 
                    * (dlon_half).sin().powi(2));
        let c: T = a.sqrt().atan2((T::one() - a).sqrt());

        <f64 as Into<T>>::into(2.0 * SEMI_AXIS_MAJOR) * c
    }

    pub fn from_rae(rae: RAEPoint<T>, reference: LLAPoint<T>) -> Self
    where T: std::convert::From<f64> {
        let xyz: XYZPoint<T> = rae.into();
        xyz.to_ecef(reference.into()).into()
    }

    pub fn to_rae(&self, reference: LLAPoint<T>) -> RAEPoint<T> 
    where T: std::convert::From<f64> {
        let xyz: XYZPoint<T> = self.clone().to_xyz(reference.into());
        xyz.into()
    }

    pub fn from_xyz(xyz: XYZPoint<T>, reference: LLAPoint<T>) -> Self
    where T: std::convert::From<f64> {
        let ecef: ECEFPoint<T> = xyz.to_ecef(reference.into());
        ecef.into()
    }

    pub fn to_xyz(&self, reference: LLAPoint<T>) -> XYZPoint<T> 
    where T: std::convert::From<f64> {
        let ecef: ECEFPoint<T> = self.clone().into();
        ecef.to_xyz(reference.into())
    }
}

impl<T: Float> From<XYZPoint<T>> for RAEPoint<T> {
    fn from(xyz: XYZPoint<T>) -> Self {
        let range = (xyz.x.powi(2) + xyz.y.powi(2) + xyz.z.powi(2)).sqrt();
        let azimuth = xyz.y.atan2(xyz.x);
        let elevation = xyz.z.atan2((xyz.x.powi(2) + xyz.y.powi(2)).sqrt());
        RAEPoint::new(range, azimuth, elevation)
    }
}

impl<T: Float> From<RAEPoint<T>> for XYZPoint<T> {
    fn from(rae: RAEPoint<T>) -> Self {
        let x = rae.range * rae.azimuth.to_radians().sin() * rae.elevation.to_radians().cos();
        let y = rae.range * rae.azimuth.to_radians().cos() * rae.elevation.to_radians().cos();
        let z = rae.range * rae.elevation.to_radians().sin();
        XYZPoint { x, y, z }
    }
}

impl<T: Float + std::ops::Mul<Output = T>> From<LLAPoint<T>> for ECEFPoint<T> where f64: Into<T> {    
    fn from(coord: LLAPoint<T>) -> Self {
        let radius_local: T = SEMI_AXIS_MAJOR.into() / (1.0_f64.into() - ECCENTRICITY_SQUARE.into() * coord.latitude.to_radians().sin().powi(2))
                           + coord.altitude;
        let x_ecef = radius_local * coord.longitude.to_radians().cos() * coord.latitude.to_radians().cos();
        let y_ecef = radius_local * coord.longitude.to_radians().sin() * coord.latitude.to_radians().cos();
        let z_ecef = radius_local * (1.0 - ECCENTRICITY_SQUARE).into() * coord.latitude.to_radians().sin();
        ECEFPoint::new(x_ecef, y_ecef, z_ecef)
    }
}

impl <T: Float + std::ops::Mul<Output = T>> From<ECEFPoint<T>> for LLAPoint<T> where f64: Into<T> {
    fn from(coord: ECEFPoint<T>) -> Self {
        let radius = (coord.x_ecef.powi(2) + coord.y_ecef.powi(2)).sqrt();
        let longitude = coord.y_ecef.atan2(coord.x_ecef);
        let mut latitude = coord.z_ecef.atan2(radius* (1.0 - ECCENTRICITY_SQUARE).into());
        let mut old_latitude = 0.0.into();
        while (latitude - old_latitude).abs() > 1e-10.into() {
            old_latitude = latitude;
            let n = SEMI_AXIS_MAJOR.into() / ((1.0_f64.into() - ECCENTRICITY_SQUARE.into() * latitude.sin().powi(2)).sqrt());
            latitude = (coord.z_ecef + (ECCENTRICITY_SQUARE * SEMI_AXIS_MINOR).into() * n * latitude.sin()).atan2(radius);
        }
        let n = SEMI_AXIS_MAJOR.into() / ((1.0_f64.into() - ECCENTRICITY_SQUARE.into() * latitude.sin().powi(2)).sqrt());
        
        let altitude = radius / latitude.cos() - n;

        LLAPoint::new(latitude.to_degrees(), longitude.to_degrees(), altitude)
    }
}

