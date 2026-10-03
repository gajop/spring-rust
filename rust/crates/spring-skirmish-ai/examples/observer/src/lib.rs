use spring_skirmish_ai::{Context, Event, Result, SkirmishAi};
struct Observer;
impl SkirmishAi for Observer {
    fn new(context:&Context<'_>)->Result<Self> {context.log("Rust Observer initialized")?;Ok(Self)}
    fn handle(&mut self,context:&Context<'_>,event:Event<'_>)->Result<()> {
        if let Event::Update {frame}=event {
            if frame%300==0 {context.log(&format!("frame={frame} own={} visible={}",context.owned_units()?.len(),context.visible_enemies()?.len()))?;}
        }
        Ok(())
    }
}
spring_skirmish_ai::export_ai!(Observer);
