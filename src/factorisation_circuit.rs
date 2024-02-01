/// Factorisation Circuit
///
/// In this example we want to prove the knowledge of two private numbers which are factors of a private number
/// (currently not exposing product on the instance, but will do once current issues are resolved).
///
use axiom_eth::{
    halo2_base::AssignedValue,
    halo2_proofs::{
        circuit::{Layouter, Value},
        plonk::{Advice, Column, ConstraintSystem},
    },
    keccak::KeccakChip,
    rlc::circuit::builder::RlcCircuitBuilder,
    utils::{
        build_utils::dummy::DummyFrom,
        component::{
            circuit::{
                ComponentBuilder, CoreBuilder, CoreBuilderOutput, CoreBuilderOutputParams,
                CoreBuilderParams,
            },
            promise_collector::PromiseCaller,
            types::{EmptyComponentType, LogicalEmpty},
        },
    },
    Field,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub struct SimpleCircuitParams;

impl CoreBuilderParams for SimpleCircuitParams {
    fn get_output_params(&self) -> CoreBuilderOutputParams {
        // TODO see what this means
        CoreBuilderOutputParams::new(vec![1])
    }
}

// Private inputs to our circuit
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SimpleCircuitInput {
    a: u64,
    b: u64,
}

impl DummyFrom<SimpleCircuitParams> for SimpleCircuitInput {
    fn dummy_from(_seed: SimpleCircuitParams) -> Self {
        SimpleCircuitInput { a: 1, b: 2 }
    }
}

// Raw halo2 configuration
#[derive(Clone)]
pub struct SimpleCircuitConfig {
    advice: Column<Advice>,
}

// TODO reason why we have a circuit component struct as well as SimpleCircuitInput
pub struct SimpleCircuit {
    input: SimpleCircuitInput,
}

impl<F: Field> ComponentBuilder<F> for SimpleCircuit {
    type Config = SimpleCircuitConfig;

    type Params = SimpleCircuitParams;

    fn new(_params: Self::Params) -> Self {
        Self {
            input: SimpleCircuitInput::default(),
        }
    }

    fn get_params(&self) -> Self::Params {
        SimpleCircuitParams
    }

    fn configure_with_params(
        meta: &mut ConstraintSystem<F>,
        _params: Self::Params,
    ) -> Self::Config {
        // we can add raw halo2 config here
        // TODO constrain some advice to be multiplication of two advices
        SimpleCircuitConfig {
            advice: meta.advice_column(),
        }
    }

    fn calculate_params(&mut self) -> Self::Params {
        SimpleCircuitParams
    }
}

impl<F: Field> CoreBuilder<F> for SimpleCircuit {
    type CompType = EmptyComponentType<F>;

    type PublicInstanceValue = LogicalEmpty<F>;

    type PublicInstanceWitness = LogicalEmpty<AssignedValue<F>>;

    type CoreInput = SimpleCircuitInput;

    fn feed_input(&mut self, input: Self::CoreInput) -> anyhow::Result<()> {
        println!("feed_input {:?}", input);
        self.input = input;
        Ok(())
    }

    fn virtual_assign_phase0(
        &mut self,
        builder: &mut RlcCircuitBuilder<F>,
        promise_caller: PromiseCaller<F>,
    ) -> CoreBuilderOutput<F, Self::CompType> {
        println!("virtual_assign_phase0");
        let keccak =
            KeccakChip::new_with_promise_collector(builder.range_chip(), promise_caller.clone());

        let ctx = builder.base.main(0);

        println!("inputs {:?}", self.input);
        let a = ctx.load_witness(F::from(1));
        println!("load witness a: {:?}", a);
        let output = keccak.keccak_fixed_len(ctx, vec![a]);
        let output_bytes = output.output_bytes;
        println!("after");

        // we can do halo2 lib stuff here
        CoreBuilderOutput {
            public_instances: vec![],
            virtual_table: vec![],
            logical_results: vec![],
        }
    }

    fn raw_synthesize_phase0(&mut self, config: &Self::Config, layouter: &mut impl Layouter<F>) {
        println!("raw_synthesize_phase0");
        // we can do raw halo2 synthesis stuff here
        layouter
            .assign_region(
                || "myregion",
                |mut region| {
                    region.assign_advice(
                        || "advice a",
                        config.advice,
                        0,
                        || Value::known(F::from(self.input.a)),
                    )?;
                    region.assign_advice(
                        || "advice b",
                        config.advice,
                        1,
                        || Value::known(F::from(self.input.b)),
                    )?;
                    Ok(())
                },
            )
            .unwrap();
        // layouter.constrain_instance(cell.cell(), column, row)
    }

    fn virtual_assign_phase1(&mut self, _builder: &mut RlcCircuitBuilder<F>) {
        println!("virtual_assign_phase1");
    }

    fn raw_synthesize_phase1(&mut self, _config: &Self::Config, _layouter: &mut impl Layouter<F>) {
        println!("raw_synthesize_phase1");
    }
}
