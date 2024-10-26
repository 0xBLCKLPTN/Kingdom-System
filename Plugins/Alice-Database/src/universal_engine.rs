use std::fmt::Debug;
use std::io;
use std::path::{ PathBuf, Path };


pub trait DefaultDocument{
    fn update_data(&mut self, name: &str) -> io::Result<()>;
    fn delete_data(&mut self, name: &str) -> io::Result<()>;
}

pub trait DefaultCollection<D>
    where D: DefaultDocument + Debug + Clone,
{
    fn new() -> Self;
    fn get_document( &self, name: &str ) -> Option<&D>;
    fn add_document( &mut self, name: &str, content: &str ) -> io::Result<()>;
    fn delete_document( &mut self, name: &str ) -> io::Result<()>;
    fn path( &self ) -> PathBuf;
}

/*
pub trait DefaultEngine<C, D>
    where
        D: DefaultDocument + Debug + Clone,
        C: DefaultCollection<D>,
{
    type CollectionWithDocs;
    fn new(root: &Path) -> Self;
    fn get_collection_mut( &mut self, name: &str ) -> Option<&mut C>;
    fn get_collection( &self, name: &str ) -> Option<&C>;

    fn add_collection( &mut self, name: &str ) -> Option<&mut C>;
        
    fn get_document( &mut self, collection_name: &str, document_name: &str ) -> Option<&D>; 
}

#[derive(Debug, Clone)]
pub struct Document {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Collection<D> {
    pub documents: Vec<D>
}

impl DefaultDocument for Document {
    fn update_data(&mut self, name: &str) -> io::Result<()> {
        Ok(())
    }

    fn delete_data(&mut self, name: &str) -> io::Result<()> {
        Ok(())
    }

    fn 
}

impl DefaultCollection<Document> for Collection<Document> {
    fn new() -> Collection<Document> {
        todo!()
    }

    fn get_document( &self, name: &str ) -> Option<&Document> {
        todo!()
    }
    fn add_document( &mut self, name: &str, content: &str ) -> io::Result<()> {
        todo!()
    }

    fn delete_document( &mut self, name: &str ) -> io::Result<()> {
        todo!()
    }
    
    fn path( &self ) -> PathBuf {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct JSONEngine<C> {
    pub collections: Vec<C>
}

impl DefaultEngine<Collection<Document>, Document> for JSONEngine<Collection<Document>> {
    type CollectionWithDocs = Collection<Document>;

    fn new(root: &Path) -> JSONEngine<Self::CollectionWithDocs> {
        todo!()
    }
    fn get_collection_mut(&mut self, name: &str) -> Option<&mut Self::CollectionWithDocs> {
        todo!()
    }
    fn get_collection(&self, name: &str) -> Option<&Self::CollectionWithDocs> {
        todo!()
    }
    fn add_collection(&mut self, name: &str) -> Option<&mut Self::CollectionWithDocs> {
        todo!()
    }
    fn get_document(&mut self, collection_name: &str, document_name: &str) -> Option<&Document> {
        todo!()
    }

}
*/
