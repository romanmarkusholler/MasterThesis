use std::mem::size_of;
use rounded_div::RoundedDiv;
use winterfell::math::{FieldElement, StarkField};
use winterfell::{FieldExtension, HashFunction, ProofOptions};
use rand::Rng;

// index definition helper
pub struct IndexDefinition {
    pub idx: usize,
    pub size: usize,
}

impl IndexDefinition {
    pub fn begin(&self) -> usize {
        self.idx
    }
    pub fn end(&self) -> usize {
        self.idx + self.size
    }
}

pub trait IndexDefinitionSlice<T> {
    fn id_slice(&self, id: IndexDefinition) -> &[T];
}

impl<T> IndexDefinitionSlice<T> for &[T] {
    fn id_slice(&self, id: IndexDefinition) -> &[T] {
        let result = &self[id.begin()..id.end()];
        result
    }
}

// TODO allow &mut [T] to be sliced as well

pub const fn next_power_of_two(number: usize) -> usize {
    let mut result = 1;
    let mut ctr = 0usize;
    while ctr < size_of::<usize>() * 8 {
        if result >= number {
            break
        }
        result *= 2;
        ctr += 1;
    }
    result
}


pub fn create_meta(mut num: usize) -> Vec<u8> {
    let mut result = vec![];
    for _ in 0..size_of::<usize>() {
        let tmp = (num & 0xFF) as u8;
        result.push(tmp);
        num >>= 8;
    }
    result
}

pub fn get_meta(meta: &Vec<u8>) -> usize {
    let mut result = 0 as usize;
    for i in 0..meta.len() {
        result += (meta[i] as usize) << (i * 8);
    }
    result
}

pub struct PlainStatisticsU128<E: FieldElement> {
    pub sum: u128,
    pub sum_e: E,
    pub var: u128,
    pub var_e: E,
    pub avg_rounded: u128,
    pub avg_rounded_e: E,
    pub min: u128,
    pub min_e: E,
    pub max: u128,
    pub max_e: E,
    pub med_low: u128,
    pub med_low_e: E,
    pub med_high: u128,
    pub med_high_e: E,
    pub stark_std_dev: f64,
    pub real_std_dev: f64,
    pub median: f64,
    pub average: f64,
}

pub struct PlainStatisticsU64<E: FieldElement> {
    pub sum: u64,
    pub sum_e: E,
    pub var: u64,
    pub var_e: E,
    pub avg_rounded: u64,
    pub avg_rounded_e: E,
    pub min: u64,
    pub min_e: E,
    pub max: u64,
    pub max_e: E,
    pub med_low: u64,
    pub med_low_e: E,
    pub med_high: u64,
    pub med_high_e: E,
    pub stark_std_dev: f64,
    pub real_std_dev: f64,
    pub median: f64,
    pub average: f64,
}

pub fn get_stats_string_u128<E: FieldElement>(stats: &PlainStatisticsU128<E>) -> String {
    format!("Standard deviation diff is {}, stddev real: {}, stddev stark: {}, avg rounded: {}, real avg: {} min: {}, max: {}, median: {}",
             f64::abs(stats.real_std_dev - stats.stark_std_dev),
             stats.real_std_dev, stats.stark_std_dev, stats.avg_rounded, stats.average, stats.min, stats.max, stats.median)
}

pub fn get_stats_string_u64<E: FieldElement>(stats: &PlainStatisticsU64<E>) -> String {
    format!("Standard deviation diff is {}, stddev real: {}, stddev stark: {}, avg rounded: {}, real avg: {} min: {}, max: {}, median: {}",
             f64::abs(stats.real_std_dev - stats.stark_std_dev),
             stats.real_std_dev, stats.stark_std_dev, stats.avg_rounded, stats.average, stats.min, stats.max, stats.median)
}

/// range type [)
pub fn get_rand_values<E: From<u16>>(begin: u16, end: u16, length: usize) -> (Vec<u16>, Vec<E>) {
    assert!(begin < end);
    let mut result_u16 = vec![0u16; length];
    let mut rng = rand::thread_rng();
    for i in 0..length {
        result_u16[i] = rng.gen_range(begin..end) as u16;
    }
    let other = result_u16.clone();
    (result_u16, other.iter().map(|elem| E::from(*elem)).collect::<Vec<E>>())
}

pub fn get_plain_statistics_u128<E: StarkField + FieldElement<PositiveInteger = u128>>(values: Vec<u16>) -> PlainStatisticsU128<E> {
    let mut result = PlainStatisticsU128 {
        sum: 0u128,
        sum_e: E::ZERO,
        var: 0u128,
        var_e: E::ZERO,
        avg_rounded: 0u128,
        avg_rounded_e: E::ZERO,
        min: 0u128,
        min_e: E::ZERO,
        max: 0u128,
        max_e: E::ZERO,
        med_low: 0u128,
        med_low_e: E::ZERO,
        med_high: 0u128,
        med_high_e: E::ZERO,
        stark_std_dev: 0.0,
        real_std_dev: 0.0,
        median: 0.0,
        average: 0.0,
    };

    for e in &values {
        result.sum += *e as u128;
    }
    result.sum_e = E::from(result.sum);
    result.avg_rounded = result.sum.rounded_div(values.len() as u128);
    result.avg_rounded_e = E::from(result.avg_rounded);
    for e in &values {
        result.var_e += (E::from(*e as u128) - result.avg_rounded_e) * (E::from(*e as u128) - result.avg_rounded_e);
    }
    result.var = result.var_e.as_int();
    result.min = values.iter().min().unwrap().clone() as u128;
    result.max = values.iter().max().unwrap().clone() as u128;
    result.min_e = E::from(result.min);
    result.max_e = E::from(result.max);
    let mut sorted_values = values.clone();
    sorted_values.sort();
    if 0 == values.len() % 2 {
        result.med_low = sorted_values[values.len() / 2 - 1].clone() as u128;
        result.med_high = sorted_values[values.len() / 2].clone() as u128;
    } else {
        result.med_low = sorted_values[values.len() / 2].clone() as u128;
        result.med_high = sorted_values[values.len() / 2].clone() as u128;
    }
    result.med_low_e = E::from(result.med_low);
    result.med_high_e = E::from(result.med_high);
    result.stark_std_dev = f64::sqrt(result.var as f64 / (values.len() - 1) as f64);
    {
        let real_avg = result.sum as f64 / values.len() as f64;
        let mut real_var = 0f64;
        for e in &values {
            real_var += (*e as f64 - real_avg) * (*e as f64 - real_avg);
        }
        result.real_std_dev = f64::sqrt(real_var / (values.len() - 1) as f64);
    }
    result.median = (result.med_low as f64 + result.med_high as f64) / 2.;
    result.average = (result.sum as f64) / values.len() as f64;

    result
}

pub fn get_plain_statistics_u64<E: StarkField + FieldElement<PositiveInteger = u64>>(values: Vec<u16>) -> PlainStatisticsU64<E> {
    let mut result = PlainStatisticsU64 {
        sum: 0u64,
        sum_e: E::ZERO,
        var: 0u64,
        var_e: E::ZERO,
        avg_rounded: 0u64,
        avg_rounded_e: E::ZERO,
        min: 0u64,
        min_e: E::ZERO,
        max: 0u64,
        max_e: E::ZERO,
        med_low: 0u64,
        med_low_e: E::ZERO,
        med_high: 0u64,
        med_high_e: E::ZERO,
        stark_std_dev: 0.0,
        real_std_dev: 0.0,
        median: 0.0,
        average: 0.0,
    };

    for e in &values {
        result.sum += *e as u64;
    }
    result.sum_e = E::from(result.sum);
    result.avg_rounded = result.sum.rounded_div(values.len() as u64);
    result.avg_rounded_e = E::from(result.avg_rounded);
    for e in &values {
        result.var_e += (E::from(*e as u64) - result.avg_rounded_e) * (E::from(*e as u64) - result.avg_rounded_e);
    }
    result.var = result.var_e.as_int();
    result.min = values.iter().min().unwrap().clone() as u64;
    result.max = values.iter().max().unwrap().clone() as u64;
    result.min_e = E::from(result.min);
    result.max_e = E::from(result.max);
    let mut sorted_values = values.clone();
    sorted_values.sort();
    if 0 == values.len() % 2 {
        result.med_low = sorted_values[values.len() / 2 - 1].clone() as u64;
        result.med_high = sorted_values[values.len() / 2].clone() as u64;
    } else {
        result.med_low = sorted_values[values.len() / 2].clone() as u64;
        result.med_high = sorted_values[values.len() / 2].clone() as u64;
    }
    result.med_low_e = E::from(result.med_low);
    result.med_high_e = E::from(result.med_high);
    result.stark_std_dev = f64::sqrt(result.var as f64 / (values.len() - 1) as f64);
    {
        let real_avg = result.sum as f64 / values.len() as f64;
        let mut real_var = 0f64;
        for e in &values {
            real_var += (*e as f64 - real_avg) * (*e as f64 - real_avg);
        }
        result.real_std_dev = f64::sqrt(real_var / (values.len() - 1) as f64);
    }
    result.median = (result.med_low as f64 + result.med_high as f64) / 2.;
    result.average = (result.sum as f64) / values.len() as f64;

    result
}

pub fn get_proof_options(blowup_factor: usize, field_extension: FieldExtension) -> ProofOptions {
    ProofOptions::new(
        32, // number of queries
        blowup_factor,  // blowup factor
        0,  // grinding factor
        HashFunction::Blake3_256,
        field_extension,
        8,   // FRI folding factor
        128, // FRI max remainder length
    )
}